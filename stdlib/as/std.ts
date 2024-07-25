@external("env", "putchar")
declare function putchar(char: i32): void;

export function print(message: string): void {
    for (let i = 0; i < message.length; i++) {
        putchar(message.charCodeAt(i));
    }
}

export function println(message: string): void {
    print(message);
    putchar(10); // Print newline character
}
