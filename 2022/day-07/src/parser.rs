use crate::{CommandLine, LsLine};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending, space1},
    combinator::opt,
    multi::{many0, separated_list0},
    sequence::terminated,
    *,
};

fn cd_command(input: &str) -> IResult<&str, CommandLine> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir_name) = not_line_ending(input)?;

    Ok((input, CommandLine::CdCommand(dir_name.to_string())))
}

fn ls_command(input: &str) -> IResult<&str, CommandLine> {
    let (input, _) = tag("$ ls\n")(input)?;
    let (input, ls_outputs) =
        separated_list0(tag("\n"), alt((ls_dir_output, ls_file_output)))(input)?;

    Ok((input, CommandLine::LsCommand(ls_outputs)))
}

fn ls_dir_output(input: &str) -> IResult<&str, LsLine> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((input, LsLine::Dir(name.to_string())))
}

fn ls_file_output(input: &str) -> IResult<&str, LsLine> {
    let (input, file_size) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((
        input,
        LsLine::File(name.to_string(), file_size.parse().unwrap()),
    ))
}

fn command_line(input: &str) -> IResult<&str, CommandLine> {
    terminated(alt((cd_command, ls_command)), opt(newline))(input)
}

pub fn parse(input: &str) -> Vec<CommandLine> {
    many0(command_line)(input).unwrap().1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let example_command_lines = parse(crate::tests::EXAMPLE_INPUT);
        assert_eq!(example_command_lines.len(), 10);
    }

    #[test]
    fn simple_cd_works() {
        let command_lines = parse("$ cd test");
        assert_eq!(command_lines.len(), 1);
        assert_eq!(command_lines[0], CommandLine::CdCommand("test".to_string()));
    }

    #[test]
    fn simple_ls_works() {
        let command_lines = parse(
            "$ ls
dir a
14848514 b.txt",
        );
        assert_eq!(command_lines.len(), 1);
        assert_eq!(
            command_lines[0],
            CommandLine::LsCommand(vec![
                LsLine::Dir("a".to_string()),
                LsLine::File("b.txt".to_string(), 14848514),
            ])
        );
    }

    #[test]
    fn mixed_example_works() {
        let command_lines = parse(
            "$ cd ..
$ ls
dir a
$ cd a
$ ls
14848514 b.txt
",
        );
        assert_eq!(command_lines.len(), 4);
        assert_eq!(command_lines[0], CommandLine::CdCommand("..".to_string()));
        assert_eq!(
            command_lines[1],
            CommandLine::LsCommand(vec![LsLine::Dir("a".to_string()),])
        );
        assert_eq!(command_lines[2], CommandLine::CdCommand("a".to_string()));
        assert_eq!(
            command_lines[3],
            CommandLine::LsCommand(vec![LsLine::File("b.txt".to_string(), 14848514),])
        );
    }
}
