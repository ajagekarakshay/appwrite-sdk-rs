//! Runtime enum

use serde::{Deserialize, Serialize};

/// Function runtime environments
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Runtime {
    #[serde(rename = "node-14.5")]
    Node14_5,
    #[serde(rename = "node-16.0")]
    Node16_0,
    #[serde(rename = "node-18.0")]
    Node18_0,
    #[serde(rename = "node-19.0")]
    Node19_0,
    #[serde(rename = "node-20.0")]
    Node20_0,
    #[serde(rename = "node-21.0")]
    Node21_0,
    #[serde(rename = "php-8.0")]
    Php8_0,
    #[serde(rename = "php-8.1")]
    Php8_1,
    #[serde(rename = "php-8.2")]
    Php8_2,
    #[serde(rename = "php-8.3")]
    Php8_3,
    #[serde(rename = "python-3.8")]
    Python3_8,
    #[serde(rename = "python-3.9")]
    Python3_9,
    #[serde(rename = "python-3.10")]
    Python3_10,
    #[serde(rename = "python-3.11")]
    Python3_11,
    #[serde(rename = "python-3.12")]
    Python3_12,
    #[serde(rename = "ruby-3.0")]
    Ruby3_0,
    #[serde(rename = "ruby-3.1")]
    Ruby3_1,
    #[serde(rename = "ruby-3.2")]
    Ruby3_2,
    #[serde(rename = "ruby-3.3")]
    Ruby3_3,
    #[serde(rename = "go-1.19")]
    Go1_19,
    #[serde(rename = "go-1.20")]
    Go1_20,
    #[serde(rename = "go-1.21")]
    Go1_21,
    #[serde(rename = "go-1.22")]
    Go1_22,
    #[serde(rename = "dotnet-3.1")]
    Dotnet3_1,
    #[serde(rename = "dotnet-6.0")]
    Dotnet6_0,
    #[serde(rename = "dotnet-7.0")]
    Dotnet7_0,
    #[serde(rename = "java-8.0")]
    Java8_0,
    #[serde(rename = "java-11.0")]
    Java11_0,
    #[serde(rename = "java-17.0")]
    Java17_0,
    #[serde(rename = "java-18.0")]
    Java18_0,
    #[serde(rename = "java-21.0")]
    Java21_0,
    #[serde(rename = "swift-5.5")]
    Swift5_5,
    #[serde(rename = "swift-5.8")]
    Swift5_8,
    #[serde(rename = "swift-5.9")]
    Swift5_9,
    #[serde(rename = "kotlin-1.6")]
    Kotlin1_6,
    #[serde(rename = "kotlin-1.8")]
    Kotlin1_8,
    #[serde(rename = "kotlin-1.9")]
    Kotlin1_9,
    #[serde(rename = "cpp-17")]
    Cpp17,
    #[serde(rename = "cpp-20")]
    Cpp20,
    #[serde(rename = "dart-2.15")]
    Dart2_15,
    #[serde(rename = "dart-2.16")]
    Dart2_16,
    #[serde(rename = "dart-2.17")]
    Dart2_17,
    #[serde(rename = "dart-2.18")]
    Dart2_18,
    #[serde(rename = "dart-3.0")]
    Dart3_0,
    #[serde(rename = "dart-3.1")]
    Dart3_1,
    #[serde(rename = "dart-3.3")]
    Dart3_3,
    #[serde(rename = "deno-1.21")]
    Deno1_21,
    #[serde(rename = "deno-1.24")]
    Deno1_24,
    #[serde(rename = "deno-1.35")]
    Deno1_35,
    #[serde(rename = "deno-1.40")]
    Deno1_40,
    #[serde(rename = "bun-1.0")]
    Bun1_0,
}

impl AsRef<str> for Runtime {
    fn as_ref(&self) -> &str {
        match self {
            Runtime::Node14_5 => "node-14.5",
            Runtime::Node16_0 => "node-16.0",
            Runtime::Node18_0 => "node-18.0",
            Runtime::Node19_0 => "node-19.0",
            Runtime::Node20_0 => "node-20.0",
            Runtime::Node21_0 => "node-21.0",
            Runtime::Php8_0 => "php-8.0",
            Runtime::Php8_1 => "php-8.1",
            Runtime::Php8_2 => "php-8.2",
            Runtime::Php8_3 => "php-8.3",
            Runtime::Python3_8 => "python-3.8",
            Runtime::Python3_9 => "python-3.9",
            Runtime::Python3_10 => "python-3.10",
            Runtime::Python3_11 => "python-3.11",
            Runtime::Python3_12 => "python-3.12",
            Runtime::Ruby3_0 => "ruby-3.0",
            Runtime::Ruby3_1 => "ruby-3.1",
            Runtime::Ruby3_2 => "ruby-3.2",
            Runtime::Ruby3_3 => "ruby-3.3",
            Runtime::Go1_19 => "go-1.19",
            Runtime::Go1_20 => "go-1.20",
            Runtime::Go1_21 => "go-1.21",
            Runtime::Go1_22 => "go-1.22",
            Runtime::Dotnet3_1 => "dotnet-3.1",
            Runtime::Dotnet6_0 => "dotnet-6.0",
            Runtime::Dotnet7_0 => "dotnet-7.0",
            Runtime::Java8_0 => "java-8.0",
            Runtime::Java11_0 => "java-11.0",
            Runtime::Java17_0 => "java-17.0",
            Runtime::Java18_0 => "java-18.0",
            Runtime::Java21_0 => "java-21.0",
            Runtime::Swift5_5 => "swift-5.5",
            Runtime::Swift5_8 => "swift-5.8",
            Runtime::Swift5_9 => "swift-5.9",
            Runtime::Kotlin1_6 => "kotlin-1.6",
            Runtime::Kotlin1_8 => "kotlin-1.8",
            Runtime::Kotlin1_9 => "kotlin-1.9",
            Runtime::Cpp17 => "cpp-17",
            Runtime::Cpp20 => "cpp-20",
            Runtime::Dart2_15 => "dart-2.15",
            Runtime::Dart2_16 => "dart-2.16",
            Runtime::Dart2_17 => "dart-2.17",
            Runtime::Dart2_18 => "dart-2.18",
            Runtime::Dart3_0 => "dart-3.0",
            Runtime::Dart3_1 => "dart-3.1",
            Runtime::Dart3_3 => "dart-3.3",
            Runtime::Deno1_21 => "deno-1.21",
            Runtime::Deno1_24 => "deno-1.24",
            Runtime::Deno1_35 => "deno-1.35",
            Runtime::Deno1_40 => "deno-1.40",
            Runtime::Bun1_0 => "bun-1.0",
        }
    }
}