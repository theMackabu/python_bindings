from rust import eval_javascript

def parse(json):
    return eval_javascript(f"JSON.parse({json})")

def dump(json):
    return eval_javascript(f"JSON.stringify({json})")
