(component
  (type (;0;)
    (instance
      (export (;0;) "a" (type (sub resource)))
      (type (;1;) (own 0))
      (type (;2;) (func (result 1)))
      (export (;0;) "[constructor]a" (func (type 2)))
      (export (;1;) "[static]a.other-new" (func (type 2)))
    )
  )
  (import "foo" (instance (;0;) (type 0)))
  (core module (;0;)
    (type (;0;) (func (result i32)))
    (type (;1;) (func (param i32)))
    (import "foo" "[constructor]a" (func (;0;) (type 0)))
    (import "foo" "[static]a.other-new" (func (;1;) (type 0)))
    (import "foo" "[resource-drop-own]a" (func (;2;) (type 1)))
    (import "foo" "[resource-drop-borrow]a" (func (;3;) (type 1)))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (alias export 0 "a" (type (;1;)))
  (type (;2;) (own 1))
  (core func (;0;) (canon resource.drop 2))
  (alias export 0 "a" (type (;3;)))
  (type (;4;) (borrow 3))
  (core func (;1;) (canon resource.drop 4))
  (alias export 0 "[constructor]a" (func (;0;)))
  (core func (;2;) (canon lower (func 0)))
  (alias export 0 "[static]a.other-new" (func (;1;)))
  (core func (;3;) (canon lower (func 1)))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
  (core instance (;0;)
    (export "[resource-drop-own]a" (func 0))
    (export "[resource-drop-borrow]a" (func 1))
    (export "[constructor]a" (func 2))
    (export "[static]a.other-new" (func 3))
  )
  (core instance (;1;) (instantiate 0
      (with "foo" (instance 0))
    )
  )
)