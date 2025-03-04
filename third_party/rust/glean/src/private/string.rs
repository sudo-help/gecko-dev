// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use inherent::inherent;
use std::sync::Arc;

use glean_core::metrics::MetricType;
use glean_core::ErrorType;

use crate::dispatcher;

// We need to wrap the glean-core type, otherwise if we try to implement
// the trait for the metric in `glean_core::metrics` we hit error[E0117]:
// only traits defined in the current crate can be implemented for arbitrary
// types.

/// This implements the developer facing API for recording string metrics.
///
/// Instances of this class type are automatically generated by the parsers
/// at build time, allowing developers to record values that were previously
/// registered in the metrics.yaml file.
#[derive(Clone)]
pub struct StringMetric(pub(crate) Arc<glean_core::metrics::StringMetric>);

impl StringMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: glean_core::CommonMetricData) -> Self {
        Self(Arc::new(glean_core::metrics::StringMetric::new(meta)))
    }
}

#[inherent(pub)]
impl glean_core::traits::String for StringMetric {
    fn set<S: Into<std::string::String>>(&self, value: S) {
        let metric = Arc::clone(&self.0);
        let new_value = value.into();
        dispatcher::launch(move || crate::with_glean(|glean| metric.set(glean, new_value)));
    }

    fn test_get_value<'a, S: Into<Option<&'a str>>>(
        &self,
        ping_name: S,
    ) -> Option<std::string::String> {
        crate::block_on_dispatcher();

        let queried_ping_name = ping_name
            .into()
            .unwrap_or_else(|| &self.0.meta().send_in_pings[0]);

        crate::with_glean(|glean| self.0.test_get_value(glean, queried_ping_name))
    }

    fn test_get_num_recorded_errors<'a, S: Into<Option<&'a str>>>(
        &self,
        error: ErrorType,
        ping_name: S,
    ) -> i32 {
        crate::block_on_dispatcher();

        crate::with_glean_mut(|glean| {
            glean_core::test_get_num_recorded_errors(&glean, self.0.meta(), error, ping_name.into())
                .unwrap_or(0)
        })
    }
}
