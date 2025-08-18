use once_cell::sync::Lazy;

pub static LAMBDA_STDLIB: Lazy<Vec<(&str, &str)>> = Lazy::new(|| {
    let mut list = Vec::new();
    list.push(("Any.ld", include_str!("../definitions/lambda/lang/Any.ld")));
    list.push(("CharSequence.ld", include_str!("../definitions/lambda/lang/CharSequence.ld")));
    list.push(("String.ld", include_str!("../definitions/lambda/lang/String.ld")));
    list
});
