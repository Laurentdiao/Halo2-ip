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
    pub p: f64,
    pub n: f64,
    pub x1: f64,
    pub x2: f64,
    pub x3: f64,
    pub x4: f64,
    pub x5: f64,
    pub x6: f64,
    pub x7: f64,
    pub x8: f64,
    pub x9: f64
}

fn pmean<F:ScalarField>(
    builder: &mut BaseCircuitBuilder<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
)where  F: BigPrimeField{
    const PRECISION: u32 = 63;
    println!("build_lookup_bit: {:?}", builder.lookup_bits());
    let fixed_point_chip = FixedPointChip::<F, PRECISION>::default(builder);
    let ctx = builder.main(0);

    let p = fixed_point_chip.quantization(input.p);
    let n = fixed_point_chip.quantization(input.n);
    let x1 = fixed_point_chip.quantization(input.x1);
    let x2 = fixed_point_chip.quantization(input.x2);
    let x3 = fixed_point_chip.quantization(input.x3);
    let x4 = fixed_point_chip.quantization(input.x4);
    let x5 = fixed_point_chip.quantization(input.x5);
    let x6 = fixed_point_chip.quantization(input.x6);
    let x7 = fixed_point_chip.quantization(input.x7);
    let x8 = fixed_point_chip.quantization(input.x8);
    let x9 = fixed_point_chip.quantization(input.x9);
    let con:f64 = 1.0;
    let con_1 = fixed_point_chip.quantization(con);

    let p = ctx.load_witness(p);
    let n = ctx.load_witness(n);
    let x1 = ctx.load_witness(x1);
    let x2 = ctx.load_witness(x2);
    let x3 = ctx.load_witness(x3);
    let x4 = ctx.load_witness(x4);
    let x5 = ctx.load_witness(x5);
    let x6 = ctx.load_witness(x6);
    let x7 = ctx.load_witness(x7);
    let x8 = ctx.load_witness(x8);
    let x9 = ctx.load_witness(x9);
    let con_1 = ctx.load_witness(con_1);
    make_public.extend([p, n, x1, x2, x3, x4, x5, x6, x7, x8, x9]);

    let x1p = fixed_point_chip.qpow(ctx,x1,p);
    let x2p = fixed_point_chip.qpow(ctx, x2, p);
    let x3p = fixed_point_chip.qpow(ctx, x3, p);
    let x4p = fixed_point_chip.qpow(ctx, x4, p);
    let x5p = fixed_point_chip.qpow(ctx, x5, p);
    let x6p = fixed_point_chip.qpow(ctx, x6, p);
    let x7p = fixed_point_chip.qpow(ctx, x7, p);
    let x8p = fixed_point_chip.qpow(ctx, x8, p);
    let x9p = fixed_point_chip.qpow(ctx, x9, p);

    let sum1 = fixed_point_chip.qadd(ctx, x1p,x2p);
    let sum2 = fixed_point_chip.qadd(ctx,sum1,x3p); 
    let sum3 = fixed_point_chip.qadd(ctx, sum2, x4p);
    let sum4 = fixed_point_chip.qadd(ctx, sum3, x5p);
    let sum5 = fixed_point_chip.qadd(ctx, sum4, x6p);
    let sum6 = fixed_point_chip.qadd(ctx, sum5, x7p);
    let sum7 = fixed_point_chip.qadd(ctx, sum6, x8p);
    let sum8 = fixed_point_chip.qadd(ctx, sum7, x9p);

    let mean1 = fixed_point_chip.qdiv(ctx,sum8,n);
    let q1 = fixed_point_chip.qdiv(ctx, con_1, p);
    let pmean = fixed_point_chip.qpow(ctx, mean1, q1);

    let pmean_decimal = fixed_point_chip.dequantization(*pmean.value());
    println!("pmean:{:?}",pmean_decimal);
}
//LOOKUP_BITS=8 cargo run --example pmean -- --name pmean -k 16 mock
fn main() {
    env_logger::init();

    let args = Cli::parse();
    run(pmean, args);
}