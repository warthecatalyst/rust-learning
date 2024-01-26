struct Solution {

}

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut ans = 0;
        
        for i in 0..n {
            for j in i+1..n {
                if words[i].chars().nth(0) == words[j].chars().nth(1) 
                 && words[i].chars().nth(1) == words[j].chars().nth(0) {
                    ans += 1;
                 }
            }
        }
        ans
    }
}