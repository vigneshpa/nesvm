// import { DOMParser } from "https://deno.land/x/deno_dom/deno-dom-wasm.ts";
// const parser = new DOMParser();

await wikiBooksExtract();
await extractCyclesTetanes();

const root = "./";


async function wikiBooksExtract() {

    const source = await readFile(root + "6502_Assembly.dump.txt");

    const parsed = source.split("\n\n\n")
        .map(sec => sec.split("\n\n")
            .map(blk => blk.split("\n"))
        );

    const instructions: Record<string, {
        name: string,
        opcodes: Record<string, string>
    }> = {};

    for (const sec of parsed) {
        const tables = parsetables(sec[1]);
        if (sec.length !== 2 || sec[0].length !== tables.length)
            throw new Error();
        for (let i = 0; i < sec[0].length; i++) {

            const insLine = sec[0][i];
            const table = tables[i];

            const [name, mnemonic] = insLine.split(": ");

            instructions[mnemonic] = {
                name,
                opcodes: table
            }

        }
    };

    const modes = {
        "A": "Accumulator",
        "i": "Implied",
        "#": "Immediate",
        "a": "Absolute",
        "zp": "ZeroPage",
        "r": "Relative",
        "(a)": "AbsoluteIndirect",
        "a,x": "AbsoluteIndexedWithX",
        "a,y": "AbsoluteIndexedWithY",
        "zp,x": "ZeroPageIndexedWithX",
        "zp,y": "ZeroPageIndexedWithY",
        "(zp,x)": "ZeroPageIndexedIndirect",
        "(zp),y": "ZeroPageIndirectIndexedWithY"
    } as const;

    function parsetables(tablesSource: string[]): Record<string, string>[] {
        const ret: Record<string, string>[] = [];
        let table: Record<string, string> = {};
        for (const line of tablesSource) {
            if (line === "Addressing Mode\tOpcode") {
                push();
                continue;
            };
            const pair = line.split("\t");
            if (pair.length !== 2)
                throw new Error();
            table[pair[0]] = pair[1];
        }
        push();
        return ret;
        function push() {
            if (Object.keys(table).length) {
                ret.push(table);
                table = {};
            }
        }
    };

    const final = {
        modes,
        instructions
    }
    await writeJSON(root + "source.json", final);
}

async function extractCyclesTetanes() {

    const source = await readFile(root + "tetanes.txt");

    const regex = /Instr\(0x([0-9A-F]{2})\s*,\s*([A-Z0-9]{3})\s*,\s*([A-Z]{3})\s*,\s*([0-9]+)\s*\)/g

    const matches = [...source.matchAll(regex)];

    const cycles: Record<string, number> = {};

    for (const match of matches) {
        const opc = match[1];
        const cyc = parseInt(match[4]);
        cycles[opc] = cyc;
    }

    await writeJSON(root + "cycles.json", cycles);
};

async function readFile(source: string) {
    const utf8 = new TextDecoder("UTF-8");
    const source_raw = await Deno.readFile(source);
    return utf8.decode(source_raw);
}

function writeFile(sink: string, data: string) {
    const encoder = new TextEncoder();
    return Deno.writeFile(sink, encoder.encode(data));
}

async function readJSON(source: string) {
    const data = await readFile(source);
    return JSON.parse(data);
}

// deno-lint-ignore no-explicit-any
function writeJSON(sink: string, data: any) {
    const str = JSON.stringify(data, null, 4);
    return writeFile(sink, str);
}