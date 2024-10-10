
| 演算子          | トレイト                      | メソッド名       | 説明                                  |
|-----------------|-------------------------------|------------------|---------------------------------------|
| `+`             | `std::ops::Add`               | `add`            | 加算演算子                          |
| `-`             | `std::ops::Sub`               | `sub`            | 減算演算子                          |
| `*`             | `std::ops::Mul`               | `mul`            | 乗算演算子                          |
| `/`             | `std::ops::Div`               | `div`            | 除算演算子                          |
| `%`             | `std::ops::Rem`               | `rem`            | 剰余演算子                          |
| `&`             | `std::ops::BitAnd`            | `bitand`         | ビット単位の AND 演算子             |
| `|`             | `std::ops::BitOr`             | `bitor`          | ビット単位の OR 演算子              |
| `^`             | `std::ops::BitXor`            | `bitxor`         | ビット単位の XOR 演算子             |
| `<<`            | `std::ops::Shl`               | `shl`            | 左シフト演算子                      |
| `>>`            | `std::ops::Shr`               | `shr`            | 右シフト演算子                      |
| `!`             | `std::ops::Not`               | `not`            | ビット単位の NOT 演算子             |
| `-`（単項）     | `std::ops::Neg`               | `neg`            | 単項の負の演算子                    |
| `==`            | `std::cmp::PartialEq`         | `eq`             | 等価演算子                          |
| `!=`            | `std::cmp::PartialEq`         | `ne`             | 非等価演算子                        |
| `<`             | `std::cmp::PartialOrd`        | `lt`             | 小なり演算子                        |
| `<=`            | `std::cmp::PartialOrd`        | `le`             | 以下演算子                          |
| `>`             | `std::cmp::PartialOrd`        | `gt`             | 大なり演算子                        |
| `>=`            | `std::cmp::PartialOrd`        | `ge`             | 以上演算子                          |
| `*`（参照外し） | `std::ops::Deref`             | `deref`          | 参照外し演算子（`*x`）              |
| `*`（可変参照外し） | `std::ops::DerefMut`       | `deref_mut`      | 可変参照外し演算子（`*x`）          |
| `()`            | `std::ops::Fn`                | `call`           | 関数呼び出し演算子（関数型のクロージャ用）|
| `()`（可変）    | `std::ops::FnMut`             | `call_mut`       | 可変な関数呼び出し演算子            |
| `()`（消費）    | `std::ops::FnOnce`            | `call_once`      | 所有権を持った関数呼び出し演算子    |
| `[]`            | `std::ops::Index`             | `index`          | インデックス演算子（`a[b]`）        |
| `[]`（可変）    | `std::ops::IndexMut`          | `index_mut`      | 可変なインデックス演算子（`a[b]`）  |
| `+=`            | `std::ops::AddAssign`         | `add_assign`     | 加算代入演算子                      |
| `-=`            | `std::ops::SubAssign`         | `sub_assign`     | 減算代入演算子                      |
| `*=`            | `std::ops::MulAssign`         | `mul_assign`     | 乗算代入演算子                      |
| `/=`            | `std::ops::DivAssign`         | `div_assign`     | 除算代入演算子                      |
| `%=`            | `std::ops::RemAssign`         | `rem_assign`     | 剰余代入演算子                      |
| `&=`            | `std::ops::BitAndAssign`      | `bitand_assign`  | ビット単位の AND 代入演算子         |
| `|=`            | `std::ops::BitOrAssign`       | `bitor_assign`   | ビット単位の OR 代入演算子          |
| `^=`            | `std::ops::BitXorAssign`      | `bitxor_assign`  | ビット単位の XOR 代入演算子         |
| `<<=`           | `std::ops::ShlAssign`         | `shl_assign`     | 左シフト代入演算子                  |
| `>>=`           | `std::ops::ShrAssign`         | `shr_assign`     | 右シフト代入演算子                  |
