use std::error::Error;

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}


pub fn build_command(arguments: Vec<String>) -> String {
    let mut arguments = arguments;
    arguments.remove(0);
    let command = arguments.join(" ");
    String::from(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn builds_command() {
        // is .to_string really necessary or is there a better way?
        let help_one = vec_of_strings![""];
        let help_two = vec_of_strings!["raptr", "-h"];
        
        let start_one = vec_of_strings!["raptr", "hatch"];
        let start_two = vec_of_strings!["raptr", "hatch", "3000"];

        let publish_one = vec_of_strings!["raptr", "publish"];
        let publish_two = vec_of_strings!["raptr", "publish", "/output"];
        let publish_three = vec_of_strings!["raptr", "publish", "web"];
        let publish_four = vec_of_strings!["raptr", "publish", "web", "/var/www/html"];

        let config = vec_of_strings!["raptr", "config", "something=value"];

        assert_eq!("", build_command(help_one));
        assert_eq!("-h", build_command(help_two));

        assert_eq!("hatch", build_command(start_one));
        assert_eq!("hatch 3000", build_command(start_two));

        assert_eq!("publish", build_command(publish_one));
        assert_eq!("publish /output", build_command(publish_two));
        assert_eq!("publish web", build_command(publish_three));
        assert_eq!("publish web /var/www/html", build_command(publish_four));

        assert_eq!("config something=value", build_command(config));
    }
}