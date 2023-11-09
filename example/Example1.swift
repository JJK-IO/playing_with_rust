import Foundation

class Example1 {
    var message: String

    init() {
        message = "Hello, World!"
    }

    func greet() {
        print("Greeting: \(message)")
    }

    func calculateSum(a: Int, b: Int) -> Int {
        print("Calculating sum of \(a) and \(b)")
        return a + b
    }

    func displayQuote() {
        let quote = "To be or not to be"
        print(quote)
    }
}

let example = Example1()
example.greet()
let sum = example.calculateSum(a: 5, b: 10)
print("Sum is \(sum)")
example.displayQuote()
