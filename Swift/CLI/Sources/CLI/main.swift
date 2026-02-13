import CLink

public protocol LifeStructure {
    init(w: Int32, h: Int32)
    func free()
    mutating func randomize()
}

extension UnsafeMutablePointer where Pointee: LifeStructure {
    public init(w: Int32, h: Int32) {
        self = .allocate(capacity: 1)
        self.initialize(to: Pointee(w: w, h: h))
    }

    public func free() {
        self.pointee.free()
        self.deallocate()
    }

    public func randomize() {
        self.pointee.randomize()
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

    public mutating func randomize() {
        for _ in 0..<50 {
            Set(
                &self,
                Int32.random(in: 0..<self.w),
                Int32.random(in: 0..<self.h),
                true,
            )
        }
    }
}

extension Life: LifeStructure, @retroactive CustomStringConvertible {
    public init(w: Int32, h: Int32) {
        self.init(a: .init(w: w, h: h), b: .init(w: w, h: h), w: w, h: h)
    }

    public func free() {
        a.free()
        b.free()
    }

    public func randomize() {
        self.a.randomize()
    }

    public var description: String {
        self.a.description
    }
}

extension UnsafeMutablePointer: @retroactive CustomStringConvertible where Pointee == Field {
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

// Start the game
print("Conway's Game of Life")

var l = Life(w: 40, h: 15)
defer { l.free() }

l.randomize()

print("\u{001B}7", terminator: "")

for _ in 0..<300 {
    Step(&l)
    print("\u{001B}8", terminator: "")
    print(l)
    usleep(1_000_000 / 3)
}
