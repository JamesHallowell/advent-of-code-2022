use crate::Input;

pub(crate) fn parse_input(s: &str) -> Input<'_> {
    let mut iter = s.split_whitespace().skip(1);
    let valve = iter.next().unwrap();

    let mut iter = iter.skip(2);
    let (_, flow_rate) = iter.next().and_then(|s| s.split_once("=")).unwrap();
    let flow_rate = flow_rate.trim_end_matches(';');

    let tunnels = iter.skip(4).map(|s| s.trim_end_matches(',')).collect();

    Input {
        valve,
        flow_rate: flow_rate.parse().unwrap(),
        tunnels,
    }
}
