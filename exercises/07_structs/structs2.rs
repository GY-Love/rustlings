#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // 主函数可以留空或者在这里实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            year: order_template.year, // 使用模板的年份
            made_by_phone: order_template.made_by_phone, // 使用模板的联系方式
            made_by_mobile: order_template.made_by_mobile,
            made_by_email: order_template.made_by_email,
            item_number: order_template.item_number, // 使用模板的商品编号
            count: 1, // 你的订单数量
        };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, 2019);
        assert_eq!(your_order.made_by_phone, false);
        assert_eq!(your_order.made_by_mobile, false);
        assert_eq!(your_order.made_by_email, true);
        assert_eq!(your_order.item_number, 123);
        assert_eq!(your_order.count, 1);
    }
}