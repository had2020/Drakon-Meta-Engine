struct Program {
    // 8 instructions packed into two 32-bit words (4 bytes each).
    // Word 0: Inst 0, 1, 2, 3
    // Word 1: Inst 4, 5, 6, 7
    // Each byte: [4-bit Opcode][2-bit Dest Reg][2-bit Src Reg]
    inst_pack_0: u32,
    inst_pack_1: u32,
};

struct ExecutionContext {
    regs: vec4<u32>,
};

fn execute_instruction(op: u32, dest_val: u32, src_val: u32) -> u32 {
    // mask shift amounts to 5 bits (0-31) to keep ISA stability on GPU vector ALUs
    let shift_amt = src_val & 31u;

    switch op {
        case 0u: { return select(dest_val, src_val, dest_val != 0u); } // CMOV: If dest != 0 then src else dest
        case 1u: { return min(dest_val, src_val); }                   // MIN
        case 2u: { return dest_val + src_val; }                       // IMM (a + b as defined)
        case 3u: { return src_val; }                                  // MOV
        case 4u: { return dest_val & src_val; }                       // AND
        case 5u: { return dest_val | src_val; }                       // OR
        case 6u: { return ~dest_val; }                                // NOT
        case 7u: { return dest_val ^ src_val; }                       // XOR
        case 8u: { return dest_val >> shift_amt; }                    // SHR
        case 9u: { return dest_val << shift_amt; }                    // SHL
        case 10u: { return reverseBits(reverseBits(dest_val) << shift_amt) | (dest_val >> (32u - shift_amt)); } // ROR fallback / use insertBits
        case 11u: { return (dest_val << shift_amt) | (dest_val >> ((32u - shift_amt) & 31u)); }                  // ROL
        case 12u: { return dest_val + src_val; }                       // ADD (wrapping is default in WGSL)
        case 13u: { return dest_val - src_val; }                       // SUB
        case 14u: { return max(dest_val, src_val); }                   // MAX
        case 15u: { return countOneBits(src_val); }                    // POPCNT
        default: { return dest_val; }
    }
}

fn bitmixer(counter: u32) -> u32 {
    var x: u32 = counter * 0x9e3779b9u;
    x ^= x >> 16u;
    x *= 0x85ebca6bu;
    x ^= x >> 13u;
    x *= 0xc2b2ae35u;
    x ^= x >> 16u;
    return x;
}

@group(0) @binding(0)
var<storage, read> input: array<u32>;
@group(0) @binding(1)
var<storage, read_write> output: array<u32>;

@compute @workgroup_size(64)
fn MetaDrakon(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let vm_id = (global_id.x + global_id.y + global_id.z);
    let registers = vec4<u32>(input[0], input[1], input[2], input[3]);

    /*
  let index = global_id.x;

  let array_length = arrayLength(&input);
  if global_id.x >= array_length {
      return;
  }

  output[global_id.x] = input[global_id.x] * 2;
  */
    //output[global_id.x] = input[global_id.x];
    //output[global_id.x] = arrayLength(&input);
}
