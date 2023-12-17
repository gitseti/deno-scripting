class ClientPool {
    #rid = 0;
    constructor(
        client_amount = 1,
        url = "localhost",
        port = 1883,
    ) {
        this.#rid = Deno.core.ops.op_client_pool_new(client_amount)
    }

    connect(connects_per_second) {
        Deno.core.ops.op_client_pool_connect(this.#rid, connects_per_second)
    }

    publish(topic, message) {
        Deno.core.ops.op_client_pool_publish(this.#rid, topic, message)
    }
}

((globalThis) => {
    const core = Deno.core;

    globalThis.Custom = {
        print_foobar: () => {
            core.ops.op_print_foobar();
        }
    }

    globalThis.ClientPool = ClientPool
})(globalThis);

