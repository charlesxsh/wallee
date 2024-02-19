use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr::NonNull;

#[repr(transparent)]
pub struct OwnPtr<T>
where
    T: ?Sized,
{
    pub ptr: NonNull<T>,
}

unsafe impl<T> Send for OwnPtr<T> where T: ?Sized {}

unsafe impl<T> Sync for OwnPtr<T> where T: ?Sized {}

impl<T> Copy for OwnPtr<T> where T: ?Sized {}

impl<T> Clone for OwnPtr<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> OwnPtr<T>
where
    T: ?Sized,
{
    pub fn new(ptr: Box<T>) -> Self {
        OwnPtr {
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(ptr)) },
        }
    }

    pub fn cast<U: CastTo>(self) -> OwnPtr<U::Target> {
        OwnPtr {
            ptr: self.ptr.cast(),
        }
    }

    pub unsafe fn boxed(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr.as_ptr()) }
    }

    pub fn as_ref(&self) -> RefPtr<T> {
        RefPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

    pub fn as_mut(&mut self) -> MutPtr<T> {
        MutPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }
}

#[repr(transparent)]
pub struct RefPtr<'a, T>
where
    T: ?Sized,
{
    pub ptr: NonNull<T>,
    lifetime: PhantomData<&'a T>,
}

impl<'a, T> Copy for RefPtr<'a, T> where T: ?Sized {}

impl<'a, T> Clone for RefPtr<'a, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> RefPtr<'a, T>
where
    T: ?Sized,
{
    pub fn new(ptr: &'a T) -> Self {
        RefPtr {
            ptr: NonNull::from(ptr),
            lifetime: PhantomData,
        }
    }

    #[cfg(not(anyhow_no_ptr_addr_of))]
    pub fn from_raw(ptr: NonNull<T>) -> Self {
        RefPtr {
            ptr,
            lifetime: PhantomData,
        }
    }

    pub fn cast<U: CastTo>(self) -> RefPtr<'a, U::Target> {
        RefPtr {
            ptr: self.ptr.cast(),
            lifetime: PhantomData,
        }
    }

    #[cfg(not(anyhow_no_ptr_addr_of))]
    pub fn by_mut(self) -> MutPtr<'a, T> {
        MutPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

    #[cfg(not(anyhow_no_ptr_addr_of))]
    pub fn as_ptr(self) -> *const T {
        self.ptr.as_ptr() as *const T
    }

    pub fn as_ref(self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }
}

#[repr(transparent)]
pub struct MutPtr<'a, T>
where
    T: ?Sized,
{
    pub ptr: NonNull<T>,
    lifetime: PhantomData<&'a mut T>,
}

impl<'a, T> Copy for MutPtr<'a, T> where T: ?Sized {}

impl<'a, T> Clone for MutPtr<'a, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> MutPtr<'a, T>
where
    T: ?Sized,
{
    #[cfg(anyhow_no_ptr_addr_of)]
    pub fn new(ptr: &'a mut T) -> Self {
        MutPtr {
            ptr: NonNull::from(ptr),
            lifetime: PhantomData,
        }
    }

    pub fn cast<U: CastTo>(self) -> MutPtr<'a, U::Target> {
        MutPtr {
            ptr: self.ptr.cast(),
            lifetime: PhantomData,
        }
    }

    #[cfg(not(anyhow_no_ptr_addr_of))]
    pub fn by_ref(self) -> RefPtr<'a, T> {
        RefPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

    pub fn extend<'b>(self) -> MutPtr<'b, T> {
        MutPtr {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

    pub unsafe fn deref_mut(self) -> &'a mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<'a, T> MutPtr<'a, T> {
    pub unsafe fn read(self) -> T {
        unsafe { self.ptr.as_ptr().read() }
    }
}

// Force turbofish on all calls of `.cast::<U>()`.
pub trait CastTo {
    type Target;
}

impl<T> CastTo for T {
    type Target = T;
}
