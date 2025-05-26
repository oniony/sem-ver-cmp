# Usages

* `semvercmp VER` - validates a version number
* `semvercmp LEFT RIGHT` - compares two versions
* `semvercmp LEFT OPERATOR RIGHT` - asserts relationship between LEFT and RIGHT is OPERATOR
    - `=` or `==` or `eq`
    - `!=` or `ne`
    - `<` or `lt`
    - `>` or `gt`
    - `<=` or `le`
    - `>=` or `ge`

# Cases

A == B true
    * v1.2.3 == v1.2.3
    * v1.2 == v1.2
    * v1 == v1
A == B false
    * v1.2.3 == v1.2.4  (differs on c3)
    * v1.2.3 == v1.3.3  (differs on c2)
    * v1.2.3 == v2.2.3  (differs on c1)
A != B true
    _same as A == B true_
A != B false
    _same as A == B false_
A < B true
    * v1.2.3 < v1.2.4   (S3 < S3 less on c3)
    * v1.2.3 < v1.3.3   (S3 < S3 less on c2)
    * v1.2.3 < v2.3.3   (S3 < S3 less on c1)
    * v1.2.3 < v1.2     (S3 < S2 less on c2)
    * v1.2.3 < v2.2     (S3 < S2 less on c1)
    * v1.2.3 < v2       (S3 < S1 less on c1)
    * v1.2 < v1.3.3     (S2 < S3 less on c2)
    * v1.2 < v2.3.3     (S2 < S3 less on c1)
    * v1.2 < v1.3       (S2 < S2 less on c2)
    * v1.2 < v2         (S2 < S2 less on c1)
    * v1 < v2.3.4       (S1 < S3 less on c1)
    * v1 < v2.3         (S1 < S2 less on c1)
    * v1 < v2           (S1 < S1 less on c1)
A < B false
A <= B true
A <= B false
A > B true
A > B false
A >= B true
A >= B false

## Internal Ranges

A in B true
    * v1.2.3 in v1.2.3  (S3 in S3)
    * v1.2.3 in v1.2    (S3 in S2)
    * v1.2 in v1.2      (S2 in S2)
    * v1.2.3 in v1      (S3 in S1)
    * v1.2 in v1        (S2 in S1)
    * v1 in v1          (S1 in S1)
A in B false
    * v1.2.3 in v2.3.4  (S3 in S3)
    * v1.2 in v2.3.4    (S2 in S3)
    * v1 in v2.3.4      (S1 in S3)
    * v1.2.3 in v2.3    (S3 in S2)
    * v1.2 in v2.3      (S2 in S2)
    * v1 in v2.3        (S1 in S2)
    * v1.2.3 in v2      (S3 in S1)
    * v1.2 in v2        (S2 in S1)
    * v1 in v2          (S1 in S1)

## Explicit Ranges

_Can probably wait until later_

A in [B, C, Z] true
A in [B, C, Z] false
A in [B..C] true
A in [B..C] false

# Variants

* ==, =, eq
* <, lt
* <=, le
* >, gt
* >=,  ge
* With and without v prefix
* With one, two or three parts
    - With two parts, vA.B, is the range [vA.B.0..vA.B.max]
