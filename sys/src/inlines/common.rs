pub use ::std::os::raw::{c_char, c_int, c_uint, c_void};

pub type JSValueConst = JSValue;

pub const JS_NULL: JSValue = JS_MKVAL(JS_TAG_NULL, 0);
pub const JS_UNDEFINED: JSValue = JS_MKVAL(JS_TAG_UNDEFINED, 0);
pub const JS_FALSE: JSValue = JS_MKVAL(JS_TAG_BOOL, 0);
pub const JS_TRUE: JSValue = JS_MKVAL(JS_TAG_BOOL, 1);
pub const JS_EXCEPTION: JSValue = JS_MKVAL(JS_TAG_EXCEPTION, 0);
pub const JS_UNINITIALIZED: JSValue = JS_MKVAL(JS_TAG_UNINITIALIZED, 0);

#[inline]
pub fn JS_VALUE_HAS_REF_COUNT(v: JSValue) -> bool {
    unsafe {
        JS_VALUE_GET_TAG(v) as c_uint >= JS_TAG_FIRST as c_uint
    }

}

#[inline]
pub fn JS_IsNumber(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_INT || JS_TAG_IS_FLOAT64(tag)
    }
}

#[inline]
pub fn JS_IsFloat64(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        JS_TAG_IS_FLOAT64(tag)
    }
}


#[inline]
pub fn JS_IsInt(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_INT
    }

}

#[inline]
pub fn JS_IsBigInt(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_BIG_INT
    }
}

#[inline]
pub fn JS_IsBool(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_BOOL
    }

}

#[inline]
pub fn JS_IsNull(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_NULL
    }

}

#[inline]
pub fn JS_IsUndefined(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_UNDEFINED
    }
}

#[inline]
pub fn JS_IsException(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_EXCEPTION
    }
}

#[inline]
pub fn JS_IsUninitialized(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_UNINITIALIZED
    }
}

#[inline]
pub fn JS_IsString(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_STRING
    }

}

#[inline]
pub fn JS_IsSymbol(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_SYMBOL
    }
}

#[inline]
pub fn JS_IsObject(v: JSValue) -> bool {
    unsafe {
        let tag = JS_VALUE_GET_TAG(v);
        tag == JS_TAG_OBJECT
    }
}

#[inline]
pub unsafe fn JS_ToCString(ctx: *mut JSContext, val: JSValue) -> *const c_char {
    // Type her can differ depending on architecture.
    #[allow(clippy::useless_conversion)]
    JS_ToCStringLen2(ctx, ptr::null_mut(), val, (false).into())
}
#[inline]
pub unsafe fn JS_ToCStringLen(
    ctx: *mut JSContext,
    plen: *mut usize,
    val: JSValue,
) -> *const c_char {
    // Type her can differ depending on architecture.
    #[allow(clippy::useless_conversion)]
    JS_ToCStringLen2(ctx, plen as _, val, (false).into())
}

#[inline]
pub fn JS_NewFloat64(d: f64) -> JSValue {
    union U {
        d: f64,
        u: u64,
    }

    let u = U { d };
    let val = d as i32;
    let t = U { d: val as f64 };
    /* -0 cannot be represented as integer, so we compare the bit
    representation */
    if unsafe { u.u == t.u } {
        JS_MKVAL(JS_TAG_INT, val)
    } else {
        __JS_NewFloat64(d)
    }
}
