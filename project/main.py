import rust
import v8_json

from rust import stdout, stdout_ln
from helpers import colors

json_test = "http://httpbin.org/json"
example_student = {
  "name": "John",
  "age": 16,
  "courses": ["Math", "Science"]
}

def test_js_run():
    stdout_ln(colors.RED + "\n- rust_test: " + colors.CYAN + "run javascript" + colors.END)
    file = open('tests/test_run.js', 'r')
    rust.run_javascript(file.read())
    file.close()


def test_js_eval():
    stdout_ln(colors.RED + "\n- rust_test: " + colors.CYAN + "eval javascript" + colors.END)
    file = open('tests/test_eval.js', 'r')
    data = rust.eval_javascript(file.read())
    file.close()
    stdout_ln(data)


def test_js_interop():
    stdout_ln(colors.RED + "\n- rust_test: " + colors.CYAN + "interop javascript" + colors.END)
    data = rust.eval_javascript(f"""
    const example_student = {example_student};
    example_student.age = 18;
    example_student.courses.push("Computer Science");

    JSON.stringify(example_student);
    """)
    stdout_ln(data)


def test_rust():
    stdout_ln(f"\n- {colors.RED}rust_test: {colors.CYAN}json fetch [{colors.DARKCYAN}{json_test}{colors.CYAN}]{colors.END}")
    body = rust.fetch(json_test)
    stdout_ln(v8_json.dump(body))


def main():
    # run javascript
    test_js_run()

    # eval javascript
    test_js_eval()

    # interop javascript
    test_js_interop()

    # rust functions
    test_rust()


main()
