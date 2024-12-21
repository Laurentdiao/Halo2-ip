use clap::Parser;
use halo2_base::utils::{ScalarField, BigPrimeField};
use halo2_base::AssignedValue;
use halo2_graph::gadget::fixed_point::{FixedPointChip, FixedPointInstructions};
use halo2_base::gates::circuit::builder::BaseCircuitBuilder;
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use halo2_graph::scaffold::cmd::Cli;
use halo2_graph::scaffold::run;

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct CircuitInput {
    pub x0: i8,
    pub x1: i8,
    pub x2: i8,
    pub x3: i8,
    pub x4: i8,
    pub x5: i8,
    pub x6: i8,
    pub x7: i8,
    pub x8: i8,
    pub x9: i8,
}

fn entropy<F:ScalarField>(
    builder: &mut BaseCircuitBuilder<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
)where  F: BigPrimeField{
    const PRECISION: u32 = 63;
    println!("build_lookup_bit: {:?}", builder.lookup_bits());
    let fixed_point_chip = FixedPointChip::<F, PRECISION>::default(builder);
    let ctx = builder.main(0);

    let input_values = [
        input.x0, input.x1, input.x2, input.x3, input.x4,
        input.x5, input.x6, input.x7, input.x8, input.x9,
    ];

    let (count_0, count_1) = input_values.iter().fold((0, 0), |(count_0, count_1), &x| {
        if x == 0 {
            (count_0 + 1, count_1)
        } else {
            (count_0, count_1 + 1)
        }
    });
    let total = input_values.len() as f64;
    let prob_0 = count_0 as f64 / total;
    let prob_1 = count_1 as f64 / total; 

    let dist_0 = fixed_point_chip.quantization(prob_0);
    let dist_1 = fixed_point_chip.quantization(prob_1);

    let dist_0 = ctx.load_witness(dist_0);
    let dist_1 = ctx.load_witness(dist_1);

    make_public.extend([dist_0,dist_1]);

    let logdist_0 = fixed_point_chip.qlog2(ctx,dist_0);
    let logdist_1 = fixed_point_chip.qlog2(ctx,dist_1);

    let p1 = fixed_point_chip.qmul(ctx, dist_0, logdist_0);
    let p2 = fixed_point_chip.qmul(ctx, dist_1, logdist_1);

    let res_pos = fixed_point_chip.qadd(ctx, p1, p2);
    let res = fixed_point_chip.neg(ctx,res_pos);

    let res_decimal = fixed_point_chip.dequantization(*res.value());
    println!("entropy:{:?}",res_decimal);
}
//LOOKUP_BITS=8 cargo run --example entropy -- --name entropy -k 16 mock
fn main() {
    env_logger::init();

    let args = Cli::parse();
    run(entropy, args);
}