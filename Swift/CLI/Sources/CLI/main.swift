import CLink

protocol LifeStructure {
    init(w: Int32, h: Int32)
    func free()
}

extension UnsafeMutablePointer where Pointee: LifeStructure {
    init(w: Int32, h: Int32) {
        self = .allocate(capacity: 1)
        self.initialize(to: Pointee(w: w, h: h))
    }

    func free() {
        self.pointee.free()
        self.deallocate()
    }
}

extension Field: LifeStructure {
    public init(w: Int32, h: Int32) {
        let xyfield = UnsafeMutablePointer<Bool>.allocate(capacity: Int(w * h))
        let xfield = UnsafeMutablePointer<UnsafeMutablePointer<Bool>?>.allocate(capacity: Int(w))
        for x in 0..<w {
            (xfield + Int(x)).initialize(to: (xyfield + Int(x * h)))
        }

        self.init(s: xfield, w: w, h: h)
    }

    public func free() {
        s[0]!.deallocate()
        s.deallocate()
    }
}

extension Life: LifeStructure {
    init(w: Int32, h: Int32) {
        self.init(a: .init(w: w, h: h), b: .init(w: w, h: h), w: w, h: h)
    }

    func free() {
        a.free()
        b.free()
    }
}

extension UnsafeMutablePointer: @retroactive CustomStringConvertible where Pointee == Field {
    func randomize() {
        for _ in 0..<50 {
            Set(
                self,
                Int32.random(in: 0..<self.pointee.w),
                Int32.random(in: 0..<self.pointee.h),
                true,
            )
        }
    }

    public var description: String {
        var result = ""
        for y in 0..<self.pointee.h {
            for x in 0..<self.pointee.w {
                result += Get(self, x, y) ? "*" : " "
            }
            result += "\n"
        }
        return result
    }
}

extension UnsafeMutablePointer where Pointee == Life {
    func randomize() {
        self.pointee.a.randomize()
    }
}

// Start the game
print("Conway's Game of Life")

let l = UnsafeMutablePointer<Life>(w: 40, h: 15)
defer { l.free() }

l.randomize()

print("\u{001B}7", terminator: "")

for _ in 0..<300 {
    Step(l)
    print("\u{001B}8", terminator: "")
    print(l.pointee.a!)
    usleep(1_000_000 / 3)
}
