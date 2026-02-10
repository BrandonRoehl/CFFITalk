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
    init(w: Int32, h: Int32) {
        let xyfield = UnsafeMutablePointer<Bool>.allocate(capacity: Int(w * h))
        let xfield = UnsafeMutablePointer<UnsafeMutablePointer<Bool>?>.allocate(capacity: Int(w))
        for x in 0..<w {
            (xfield + Int(x)).initialize(to: (xyfield + Int(x * h)))
        }

        self.init(s: xfield, w: w, h: h)
    }

    func free() {
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
