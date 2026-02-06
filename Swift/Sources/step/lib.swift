import CLink

@_cdecl("Step")
public func Step(_ l: UnsafeMutablePointer<Life>!) {
    // Update the state of the next field (b) from the current field (a).
    for y in 0..<l.pointee.h {
        for x in 0..<l.pointee.w {
            Set(l.pointee.b, x, y, Next(l.pointee.a, x, y))
        }
    }

    // Swap fields a and b.
    let tmp = l.pointee.a
    l.pointee.a = l.pointee.b
    l.pointee.b = tmp
}
