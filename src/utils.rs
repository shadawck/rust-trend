pub fn sanitize_response(body: &str, pos : usize) -> &str {
    let mut chars = body.chars();
    for _ in 0..pos {
        chars.next();
    }
    chars.as_str()

}