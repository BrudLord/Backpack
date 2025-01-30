pub mod logger;
pub mod metrics_service;
#[cfg(test)]
mod tests;

use knapsack_library::algorithms_service::AlgorithmsService;
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use logger::Logger;
use metrics_service::MetricService;
use metrics_service::{ExperimentUnit, MetricsUnit};
use rand::Rng;
use std::collections::HashMap;


fn main() {}
