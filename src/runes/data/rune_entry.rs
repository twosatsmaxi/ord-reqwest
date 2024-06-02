use ordinals::Terms;

#[derive(serde::Deserialize)]
pub struct RuneEntry {
    pub spaced_rune: String,
    pub mints: u128,
    pub premine: u128,
    pub divisibility: u16,
    pub number: u128,
    pub terms: Option<Terms>,
}

impl RuneEntry {
    pub fn remaining_mints(&self) -> u128 {
        if self.terms.is_none() {
            return 0;
        }
        if let Some(cap) = self.terms.unwrap().cap {
            return cap - self.mints;
        }
        return 0;
    }
    pub fn premine_percentage(&self) -> f32 {
        if self.premine == 0 {
            return 0.0;
        }
        if self.terms.is_none() {
            return 0.0;
        }
        // normalize premine by dividing it to 10 ^ divisibility
        let denominator = 10_i32.pow(self.divisibility.into());

        let premine_normalized = if denominator != 0 {
            self.premine
                .div_ceil(10_i32.pow(self.divisibility.into()) as u128)
        } else {
            self.premine
        };
        let terms = self.terms.unwrap();
        let total_mints_supply = terms.amount.unwrap() * self.mints;
        let total_mints_normalized =
            total_mints_supply.div_ceil(10_i32.pow(self.divisibility.into()) as u128);
        let circulating_supply = total_mints_normalized + premine_normalized;
        let premine_percentage = (premine_normalized * 100) as f32 / circulating_supply as f32;
        // round to 2 decimal places
        return (premine_percentage * 100.0).round() / 100.0;
    }
}
#[derive(serde::Deserialize)]
pub struct RuneResponse {
    pub entry: RuneEntry,
    pub parent: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::runes::data::rune_entry::RuneEntry;
    use super::*;

    #[test]
    fn test_premine_percentage() {
        let rune_entry = RuneEntry {
            divisibility: 2,
            spaced_rune: "Rune".to_string(),
            mints: 0,
            premine: 100,
            number: 1,
            terms: Some(Terms {
                amount: Some(1000),
                cap: Some(1000),
                ..Default::default()
            }),
        };
        assert_eq!(rune_entry.premine_percentage(), 100.0);
    }

    #[test]
    fn test_premine_percentage_nakamato() {
        let rune_entry = RuneEntry {
            divisibility: 0,
            spaced_rune: "Rune".to_string(),
            mints: 168000,
            premine: 420000000000000,
            number: 6,
            terms: Some(Terms {
                amount: Some(10000000000),
                cap: Some(168000),
                ..Default::default()
            }),
        };
        assert_eq!(rune_entry.premine_percentage(), 20.0);
    }

    #[test]
    fn test_premine_for_fehu() {
        let rune_entry = RuneEntry {
            spaced_rune: "Fehu".to_string(),
            mints: 452105,
            divisibility: 2,
            premine: 11000000000,
            number: 1,
            terms: Some(Terms {
                amount: Some(100),
                cap: Some(1111111),
                ..Default::default()
            }),
        };
        assert_eq!(rune_entry.premine_percentage(), 99.59);
    }
}