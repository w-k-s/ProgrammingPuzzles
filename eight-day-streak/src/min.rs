fn i(mut n:i32)->String{while n>1000 {n=n/1000;}let s=n.to_string();(if s.starts_with("8")||s.len()==2&&(s.starts_with("11")||s.starts_with("18")){"an"}else{"a"}).to_string()}
