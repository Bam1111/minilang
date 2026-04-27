let name: string = "MiniLang";
let version: int = 1;
print("Welcome to MiniLang!");
print(name);

let x: int = 10;
let y: int = 3;
print(x + y);
print(x * y);

if x > y {
    print("x is greater than y");
} else {
    print("y is greater than x");
}

let count: int = 0;
while count < 5 {
    print(count);
    count = count + 1;
}