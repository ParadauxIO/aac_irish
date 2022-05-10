fn concat_str(str1: String, str2: String) -> String {
    let mut string_result = str1.to_owned();
    string_result.push_str(&str2.to_owned());
    return string_result;
}