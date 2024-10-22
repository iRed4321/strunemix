use criterion::{criterion_group, criterion_main, Criterion};
use strunemix::*;

#[derive(Strunemix)]
struct LongForm {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: String,
    f: String,
    g: bool,
    h: bool,
    i: bool,
    j: f32,
    k: f64,
    l: i8,
    m: i8,
    n: i16,
    o: i32,
    p: i64,
    q: Option<String>,
    r: Option<u64>,
    s: Option<f64>,
    t: Option<i64>,
    u: (bool, Option<i32>),
    v: (bool, Option<u128>),
    w: Vec<u32>,
    x: Vec<String>,
    y: (),
    z: Option<()>
}

impl StrunemixParsableData<'_, LongFormAttrData> for LongFormAttrName {
    fn add_data(&self, data: &'_ str) -> Result<LongFormAttrData, StrunemixParseError> {
        match self {
            LongFormAttrName::A => Ok(LongFormAttrData::A(data.parse().unwrap())),
            LongFormAttrName::B => Ok(LongFormAttrData::B(data.parse().unwrap())),
            LongFormAttrName::C => Ok(LongFormAttrData::C(data.parse().unwrap())),
            LongFormAttrName::D => Ok(LongFormAttrData::D(data.parse().unwrap())),
            LongFormAttrName::E => Ok(LongFormAttrData::E(data.to_string())),
            LongFormAttrName::F => Ok(LongFormAttrData::F(data.to_string())),
            LongFormAttrName::G => Ok(LongFormAttrData::G(data.parse().unwrap())),
            LongFormAttrName::H => Ok(LongFormAttrData::H(data.parse().unwrap())),
            LongFormAttrName::I => Ok(LongFormAttrData::I(data.parse().unwrap())),
            LongFormAttrName::J => Ok(LongFormAttrData::J(data.parse().unwrap())),
            LongFormAttrName::K => Ok(LongFormAttrData::K(data.parse().unwrap())),
            LongFormAttrName::L => Ok(LongFormAttrData::L(data.parse().unwrap())),
            LongFormAttrName::M => Ok(LongFormAttrData::M(data.parse().unwrap())),
            LongFormAttrName::N => Ok(LongFormAttrData::N(data.parse().unwrap())),
            LongFormAttrName::O => Ok(LongFormAttrData::O(data.parse().unwrap())),
            LongFormAttrName::P => Ok(LongFormAttrData::P(data.parse().unwrap())),
            LongFormAttrName::Q => Ok(LongFormAttrData::Q(Some(data.to_string()))),
            LongFormAttrName::R => Ok(LongFormAttrData::R(Some(data.parse().unwrap()))),
            LongFormAttrName::S => Ok(LongFormAttrData::S(Some(data.parse().unwrap()))),
            LongFormAttrName::T => Ok(LongFormAttrData::T(Some(data.parse().unwrap()))),
            LongFormAttrName::U => {
                let mut split = data.split(',');
                let first = split.next().unwrap().parse().unwrap();
                let second = split.next().map(|x| x.parse().unwrap());
                Ok(LongFormAttrData::U((first, second)))
            },
            LongFormAttrName::V => {
                let mut split = data.split(',');
                let first = split.next().unwrap().parse().unwrap();
                let second = split.next().map(|x| x.parse().unwrap());
                Ok(LongFormAttrData::V((first, second)))
            },
            LongFormAttrName::W => {
                let mut vec = Vec::new();
                for item in data.split(',') {
                    vec.push(item.parse().unwrap());
                }
                Ok(LongFormAttrData::W(vec))
            },
            LongFormAttrName::X => {
                let mut vec = Vec::new();
                for item in data.split(',') {
                    vec.push(item.to_string());
                }
                Ok(LongFormAttrData::X(vec))
            },
            LongFormAttrName::Y => Ok(LongFormAttrData::Y(())),
            LongFormAttrName::Z => Ok(LongFormAttrData::Z(Some(()))),
        }
    }
}

fn filling() -> LongForm {
    let mut form = LongForm::empty_form::<()>();

    form.set_data_str("a", "1").unwrap();
    form.set_data_str("b", "2").unwrap();
    form.set_data_str("c", "3").unwrap();
    form.set_data_str("d", "4").unwrap();
    form.set_data_str("e", "5").unwrap();
    form.set_data_str("f", "6").unwrap();
    form.set_data_str("g", "true").unwrap();
    form.set_data_str("h", "false").unwrap();
    form.set_data_str("i", "true").unwrap();
    form.set_data_str("j", "7.0").unwrap();
    form.set_data_str("k", "8.0").unwrap();
    form.set_data_str("l", "-1").unwrap();
    form.set_data_str("m", "-2").unwrap();
    form.set_data_str("n", "-3").unwrap();
    form.set_data_str("o", "-4").unwrap();
    form.set_data_str("p", "-5").unwrap();
    form.set_data_str("q", "9").unwrap();
    form.set_data_str("r", "10").unwrap();
    form.set_data_str("s", "11.0").unwrap();
    form.set_data_str("t", "12").unwrap();
    form.set_data_str("u", "true,13").unwrap();
    form.set_data_str("v", "false,14").unwrap();
    form.set_data_str("w", "15,16,17").unwrap();
    form.set_data_str("x", "18,19,20").unwrap();
    form.set_data_str("y", "").unwrap();
    form.set_data_str("z", "").unwrap();

    let res = LongForm::from_form(form).unwrap();

    res
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("longform", |b| b.iter(|| filling()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);