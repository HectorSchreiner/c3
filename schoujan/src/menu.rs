pub struct Menu {
    pub header: String,
    pub items: Vec<Item>
}

pub struct Item {
    pub name: String,
    pub action: Box<dyn Fn()>
}

impl Item {
    pub fn new<F>(name: String, action: F) -> Self
    where
        F: Fn() + 'static, // 'static lifetime to allow storing in Box
    {
        Self {
            name,
            action: Box::new(action),
        }
    }

    pub fn execute(&self) {
        (self.action)();
    }
}

impl Menu {
    pub fn new(header: String) -> Self {
        Self {
            header,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}