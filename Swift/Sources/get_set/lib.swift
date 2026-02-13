import CLink

extension UnsafePointer where Pointee == Field {
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

@_cdecl("Get")
public func Get(_ f: UnsafePointer<Field>!, _ x: Int32, _ y: Int32) -> Bool {
    return f.cell(x: x, y: y).pointee
}

@_cdecl("Set")
public func Set(_ f: UnsafePointer<Field>!, _ x: Int32, _ y: Int32, _ b: Bool) {
    f.cell(x: x, y: y).initialize(to: b)
}
