use crate::{MathFunction, ScalarKind, TypeInner, VectorSize};

pub struct PolyfillOverload {
    pub function: PolyfilledMathFunction,
    pub width: crate::Bytes,
}

pub enum PolyfilledMathFunction {
    InverseMat2x2,
    InverseMat3x3,
    InverseMat4x4,

    OuterProduct2x2,
    OuterProduct3x3,
    OuterProduct4x4,

    OuterProduct3x2,
    OuterProduct2x3,

    OuterProduct4x2,
    OuterProduct2x4,

    OuterProduct4x3,
    OuterProduct3x4,
}

impl PolyfilledMathFunction {
    pub fn inverse_for_vecsize(dimension: VectorSize) -> PolyfilledMathFunction {
        match dimension {
            VectorSize::Bi => PolyfilledMathFunction::InverseMat2x2,
            VectorSize::Tri => PolyfilledMathFunction::InverseMat3x3,
            VectorSize::Quad => PolyfilledMathFunction::InverseMat4x4,
        }
    }

    pub fn outer_product_for_vecsize(
        columns: VectorSize,
        rows: VectorSize,
    ) -> PolyfilledMathFunction {
        match (columns, rows) {
            (VectorSize::Bi, VectorSize::Bi) => PolyfilledMathFunction::OuterProduct2x2,
            (VectorSize::Tri, VectorSize::Tri) => PolyfilledMathFunction::OuterProduct3x3,
            (VectorSize::Quad, VectorSize::Quad) => PolyfilledMathFunction::OuterProduct4x4,

            (VectorSize::Tri, VectorSize::Bi) => PolyfilledMathFunction::OuterProduct3x2,
            (VectorSize::Bi, VectorSize::Tri) => PolyfilledMathFunction::OuterProduct2x3,

            (VectorSize::Quad, VectorSize::Bi) => PolyfilledMathFunction::OuterProduct4x2,
            (VectorSize::Bi, VectorSize::Quad) => PolyfilledMathFunction::OuterProduct2x4,

            (VectorSize::Quad, VectorSize::Tri) => PolyfilledMathFunction::OuterProduct4x3,
            (VectorSize::Tri, VectorSize::Quad) => PolyfilledMathFunction::OuterProduct3x4,
        }
    }
}

impl PolyfillOverload {
    pub fn find_overload(math_function: MathFunction, ty: &TypeInner) -> Option<PolyfillOverload> {
        if math_function == MathFunction::Inverse {
            let TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } = ty
            else {
                return None;
            };
            if columns != rows || scalar.kind != ScalarKind::Float {
                return None;
            };

            Some(PolyfillOverload {
                function: PolyfilledMathFunction::inverse_for_vecsize(*columns),
                width: scalar.width,
            })
        } else if math_function == MathFunction::Outer {
            let TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } = ty
            else {
                return None;
            };
            if scalar.kind != ScalarKind::Float {
                return None;
            };
            Some(PolyfillOverload {
                function: PolyfilledMathFunction::outer_product_for_vecsize(*columns, *rows),
                width: scalar.width,
            })
        } else {
            None
        }
    }
}
