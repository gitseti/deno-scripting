let client_pool = new ClientPool(50);
client_pool.connect(50);

for (let i = 0; i < 100; i++) {
    client_pool.publish("till/" + i, "foobar")
}