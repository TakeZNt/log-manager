use chrono::{DateTime, Utc};
use serde_derive::*; // Serialize, Deserializeを使えるようにするため

/// ログを表すデータ型
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Log {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DateTime<Utc>,
}

/// 期間を表すデータ型
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

/// CSV形式のログをやりとりするためのモジュール
pub mod csv {
    /// CSV形式のログをHTTP GETで取得する際のモジュール
    pub mod get {
        use crate::DateTimeRange;

        /// 取得するためのパラメータ
        pub type Query = DateTimeRange;
    }

    /// CSV形式のログをHTTP POSTで取得する際のモジュール
    pub mod post {
        use serde_derive::*;

        /// 応答のデータ型
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub struct Response(pub usize);
    }
}

/// JSON形式のログをやりとりするためのモジュール
pub mod logs {
    /// JSON形式のログをHTTP GETで取得する際のモジュール
    pub mod get {
        use crate::{DateTimeRange, Log};
        use serde_derive::*;

        /// 要求のデータ型
        pub type Query = DateTimeRange;

        /// 応答のデータ型
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub struct Response(pub Vec<Log>);
    }

    /// JSON形式のログをHTTP POSTで取得する際のモジュール
    pub mod post {
        use chrono::{DateTime, Utc};
        use serde_derive::*;

        /// 要求のデータ型
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub struct Request {
            pub user_agent: String,
            pub response_time: i32,
            pub timestamp: Option<DateTime<Utc>>,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
