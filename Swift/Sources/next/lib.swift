import CLink

@_cdecl("Next")
public func Next(_ f: UnsafePointer<Field>!, _ x: Int32, _ y: Int32) -> Bool {
    // Count the adjacent cells that are alive.
    var alive = 0
    for i: Int32 in -1...1 {
        for j: Int32 in -1...1 {
            if (j != 0 || i != 0) && Get(f, x + i, y + j) {
                alive += 1
            }
        }
    }
    // Return next state according to the game rules:
    //   exactly 3 neighbors: on,
    //   exactly 2 neighbors: maintain current state,
    //   otherwise: off.
    return alive == 3 || alive == 2 && Get(f, x, y)
}
