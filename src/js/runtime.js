((globalThis) => {
    const core = Deno.core;

    globalThis.Custom = {
        print_foobar: () => {
            core.ops.op_print_foobar();
        }
    }
})(globalThis);

