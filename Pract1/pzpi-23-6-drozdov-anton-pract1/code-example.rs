pub trait ShippingStrategy {
    fn calculate_cost(&self, weight: f64, distance: f64) -> f64;
}

pub struct NovaPostShipping;

impl ShippingStrategy for NovaPostShipping {
    fn calculate_cost(&self, weight: f64, distance: f64) -> f64 {
        // базова ставка: 50 грн + 5 грн за кг + 0.5 грн за км
        50.0 + (5.0 * weight) + (0.5 * distance)
    }
}

pub struct UkrPostShipping;

impl ShippingStrategy for UkrPostShipping {
    fn calculate_cost(&self, weight: f64, distance: f64) -> f64 {
        // Базова ставка 30 грн + 3 грн за кг + 0.2 грн за км
        30.0 + (3.0 * weight) + (0.2 * distance)
    }
}

pub struct SelfPickupShipping;

impl ShippingStrategy for SelfPickupShipping {
    fn calculate_cost(&self, _weight: f64, _distance: f64) -> f64 {
        // Самовивіз безкоштовний
        0.0
    }
}

pub struct Order {
    weight: f64,
    distance: f64,
    shipping_strategy: Box<dyn ShippingStrategy>,
}

impl Order {
    pub fn new(weight: f64, distance: f64, strategy: Box<dyn ShippingStrategy>) -> Self {
        Self {
            weight,
            distance,
            shipping_strategy: strategy,
        }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn ShippingStrategy>) {
        self.shipping_strategy = strategy;
    }

    pub fn calculate_shipping(&self) -> f64 {
        self.shipping_strategy.calculate_cost(self.weight, self.distance)
    }
}

fn main() {
    let mut order = Order::new(10.0, 100.0, Box::new(NovaPostShipping));
    println!("Вартість доставки (Нова Пошта): {:.2} грн", order.calculate_shipping());

    order.set_strategy(Box::new(UkrPostShipping));
    println!("Вартість доставки (Укрпошта): {:.2} грн", order.calculate_shipping());

    order.set_strategy(Box::new(SelfPickupShipping));
    println!("Вартість доставки (Самовивіз): {:.2} грн", order.calculate_shipping());
}
