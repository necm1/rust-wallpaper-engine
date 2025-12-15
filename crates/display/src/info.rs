pub struct DisplayInfo {
    pub width: i32,
    pub height: i32,
    pub left: i32,
    pub top: i32,
    pub index: i32,
    pub is_primary: bool,
}

impl DisplayInfo {
    pub fn new(width: i32, height: i32, index: i32, left: i32, top: i32, is_primary: bool) -> Self {
        DisplayInfo {
            width,
            height,
            left,
            top,
            index,
            is_primary,
        }
    }
}
