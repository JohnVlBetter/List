pub enum List{
    None,
    Elem(i32, Box<List>)
}