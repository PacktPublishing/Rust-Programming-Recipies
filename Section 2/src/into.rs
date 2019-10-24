pub struct Company {
    ceo: String,
    receptionist: String,
    marketing: String,
}

pub struct CompanyIter<'a> {
    n: i8,
    c: &'a Company,
}

impl<'a> Iterator for CompanyIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        match self.n {
            1 => Some(&self.c.ceo),
            2 => Some(&self.c.receptionist),
            3 => Some(&self.c.marketing),
            _ => None,
        }
    }
}

impl<'a> IntoIterator for &'a Company {
    type Item = &'a str;
    type IntoIter = CompanyIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CompanyIter { n: 0, c: self }
    }
}

#[cfg(test)]
mod test_into {
    use super::*;
    #[test]
    fn test_into_iter() {
        let c = Company {
            ceo: "Alice".to_string(),
            receptionist: "Bob".to_string(),
            marketing: "Chad".to_string(),
        };
        let f: String = c.into_iter().fold(String::new(), |mut curr, nv| {
            curr.push_str(nv);
            curr
        });

        assert_eq!(f, "AliceBobChad");
    }
}
