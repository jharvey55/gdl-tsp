use gdl::contenders;


pub struct StopPriority {
    index: usize,
    priority: f64,
}

pub struct RouteContender {
    route_length: f64,
    route_fitness: f64,
    route: &Vec<StopPriority>,

}

impl Contender for RouteContender {
    type Fitness = f64

    fn random_solution() -> Self {
        let route = vec![StopPriority { index: 0, priority: 0.0 }];
        RouteContender {
            route_length: 0.0,
            route_fitness: 0.0,
            route,
        }
    }


}