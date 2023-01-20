struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let (mut low, mut up, mut digit, mut sp) = (false, false, false, false);
        let mut pre_ch = b'\0';
        for ch in password.bytes() {
            if ch == pre_ch {
                return false;
            }
            pre_ch = ch;
            
            if ch >= b'a' && ch <= b'z' {
                low = true;
            } else if ch >= b'A' && ch <= b'Z' {
                up = true;
            } else if ch >= b'0' && ch <= b'9' {
                digit = true;
            } else if "!@#$%^&*()-+".contains(ch as char) {
                sp = true;
            }
        }
        return low && up && digit && sp;
    }
}

fn main() {}
