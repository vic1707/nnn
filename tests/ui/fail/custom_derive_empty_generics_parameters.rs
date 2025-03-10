use nnn::nnn;

#[nnn(nnn_derive(Into<>))]
struct WrapperInto(i8);
#[nnn(nnn_derive(From<>))]
struct WrapperFrom(i8);
#[nnn(nnn_derive(TryFrom<>))]
struct WrapperTryFrom(i8);
#[nnn(nnn_derive(Borrow<>))]
struct WrapperBorrow(i8);
#[nnn(nnn_derive(AsRef<>))]
struct WrapperAsRef(i8);
#[nnn(nnn_derive(Deref<>))]
struct WrapperDeref(i8);

fn main() {}
