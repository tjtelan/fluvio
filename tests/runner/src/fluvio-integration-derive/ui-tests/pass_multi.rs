use fluvio_integration_derive::fluvio_test;
use fluvio::Fluvio;
use fluvio_test_util::test_meta::TestOption;

#[fluvio_test(min_spu = 2, topic = "test")]
pub async fn run(_client: Fluvio, _option: TestOption) {
}

fn main() {
}