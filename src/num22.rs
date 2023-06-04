pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];

    fn backtrack(s: &mut String, left: i32, right: i32, n: i32, res: &mut Vec<String>) {
        if s.len() == (n * 2) as usize {
            res.push(s.clone());
            return;
        }

        if left < n {
            s.push('(');
            backtrack(s, left + 1, right, n, res);
            s.pop();
        }

        if right < left {
            s.push(')');
            backtrack(s, left, right + 1, n, res);
            s.pop();
        }
    }

    backtrack(&mut String::new(), 0, 0, n, &mut res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(generate_parenthesis(1), vec!["()"]);
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
    }
}
