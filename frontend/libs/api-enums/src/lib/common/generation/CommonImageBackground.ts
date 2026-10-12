// NOTE: These are defined in Rust (as the source of truth) and duplicated in the frontend.
// In the future, we should use code gen (protobufs or similar) to keep the two sides in sync.

export enum CommonImageBackground {
  Auto = "auto",
  Transparent = "transparent",
  Opaque = "opaque",
}
