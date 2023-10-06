import rust

rust.run_javascript("""
runtime.fetch("http://httpbin.org/json").then(async (res) => {
   console.log(res);
});
""")

# body = rust.fetch("http://httpbin.org/json")
# rust.stdout(body)
