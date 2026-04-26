fn main()
{
	let my_string_1 = String::from("Hola que tal! ");
	let my_string_2 = String::from("Pura Vida!");
	let my_string_3 = cat_strings(&my_string_1, &my_string_2);
	println!("my_string_1: {my_string_1}");
	println!("my_string_2: {my_string_2}");
	println!("my_string_3: {my_string_3}");
}

fn cat_strings(str1: &String, str2: &String) -> String
{
	let mut my_string = String::new();
	my_string.push_str(str1);
	my_string.push_str(str2);
  my_string
}
