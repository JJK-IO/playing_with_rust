import UIKit

struct Person {
    var name: String
    var age: Int

    func introduce() {
        print("My name is \(name) and I am \(age) years old.")
    }
}

let john = Person(name: "John Doe", age: 30)
john.introduce()

func displayFavoriteColor(color: String) {
    print("My favorite color is \(color).")
}

displayFavoriteColor(color: "blue")

let warningMessage = "This is a warning message!"
print(warningMessage)

let errorMessage = "An error occurred while processing your request."
print(errorMessage)
