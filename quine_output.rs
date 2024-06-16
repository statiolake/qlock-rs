fn main(){let s="fn main(){let s=\"~\";\nfor c in s.chars()\n{match c as u8{0x7e\n=>{let s=format!(\"{s:?}\");\nprint!(\"{}\",&s[1..s.len()\n-1]);}_=>print!(\"{}\",\nc),}}}";
for c in s.chars()
{match c as u8{0x7e
=>{let s=format!("{s:?}");
print!("{}",&s[1..s.len()
-1]);}_=>print!("{}",
c),}}}