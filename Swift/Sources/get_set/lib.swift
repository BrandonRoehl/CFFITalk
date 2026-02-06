import CLink

extension UnsafeMutablePointer where Pointee == Field {
    func cell(x: Int32, y: Int32) -> UnsafeMutablePointer<Bool> {
        // If the x or y coordinates are outside the field boundaries they are wrapped
        // toroidally. For instance, an x value of -1 is treated as width-1.
        let x = Int((x + self.pointee.w) % self.pointee.w)
        let y = Int((y + self.pointee.h) % self.pointee.h)

        let xptr = self.pointee.s.advanced(by: x).pointee!
        let xyptr = xptr.advanced(by: y)

        return xyptr
    }
}

public func Get(_ f: UnsafeMutablePointer<Field>!, _ x: Int32, _ y: Int32) -> Bool {
    return f.cell(x: x, y: y).pointee
}

public func Set(_ f: UnsafeMutablePointer<Field>!, _ x: Int32, _ y: Int32, _ b: Bool) {
    return f.cell(x: x, y: y).initialize(to: b)
}
