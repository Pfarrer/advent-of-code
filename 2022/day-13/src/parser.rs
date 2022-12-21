use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as nom_u32};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated};

use crate::*;

fn list_entry(input: &str) -> IResult<&str, Entry> {
    let (input, list) = delimited(
        tag("["),
        separated_list0(tag(","), entry),
        tag("]")
    )(input)?;

    Ok((input, Entry::List(list)))
}

fn number_entry(input: &str) -> IResult<&str, Entry> {
    let (input, number) = nom_u32(input)?;

    Ok((input, Entry::Number(number)))
}

fn entry(input: &str) -> IResult<&str, Entry> {
    alt((number_entry, list_entry))(input)
}

fn package(input: &str) -> IResult<&str, Package> {
    delimited(tag("["),separated_list0(tag(","), entry), tag("]"))(input)
}

fn package_pair(input: &str) -> IResult<&str, PackagePair> {
    let (input, package1) = terminated(package, newline)(input)?;
    let (input, package2) = terminated(package, opt(newline))(input)?;

    Ok((input, (package1, package2)))
}

pub fn parse(input: &str) -> Vec<PackagePair> {
    input.split("\n\n").map(|string_pair| package_pair(string_pair).unwrap().1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_package_works() {
        let (input, package) = package("[1,1,3,2,4]").unwrap();
        assert_eq!(input.len(), 0);
        assert_eq!(package.len(), 5);
    }

    #[test]
    fn empty_package_works() {
        let (input, package) = package("[[[]]]").unwrap();
        assert_eq!(input.len(), 0);
        assert_eq!(package.len(), 1);
    }

    #[test]
    fn single_package_pair_works() {
        let (input, package_pair) = package_pair("[1]\n[2]\n").unwrap();
        assert_eq!(input.len(), 0);
        assert_eq!(package_pair.0.len(), 1);
        assert_eq!(package_pair.1.len(), 1);
    }

    #[test]
    fn flat_lists_work() {
        let package_pairs = parse("[1,1,3,1,1]\n[1,1,5,1,1]\n");
        assert_eq!(package_pairs.len(), 1);
        assert_eq!(package_pairs[0].0.len(), 5);
        assert_eq!(package_pairs[0].1.len(), 5);
    }

    #[test]
    fn nested_lists_work() {
        let package_pairs = parse("[[1],[2,3,4]]\n[[1],4]");
        assert_eq!(package_pairs.len(), 1);
        assert_eq!(package_pairs[0].0.len(), 2);
        assert_eq!(package_pairs[0].1.len(), 2);

        assert_eq!(package_pairs[0].1, vec![
            Entry::List(vec![
                Entry::Number(1)
            ]),
            Entry::Number(4)
        ]);
    }
}
