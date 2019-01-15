// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use InetAddress;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use ResolverRecordType;
use SrvTarget;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Resolver(Object<ffi::GResolver, ffi::GResolverClass, ResolverClass>);

    match fn {
        get_type => || ffi::g_resolver_get_type(),
    }
}

impl Resolver {
    //pub fn free_addresses(addresses: /*Unimplemented*/&[&Fundamental: Pointer]) {
    //    unsafe { TODO: call ffi::g_resolver_free_addresses() }
    //}

    //pub fn free_targets(targets: /*Unimplemented*/&[&Fundamental: Pointer]) {
    //    unsafe { TODO: call ffi::g_resolver_free_targets() }
    //}

    pub fn get_default() -> Option<Resolver> {
        unsafe {
            from_glib_full(ffi::g_resolver_get_default())
        }
    }
}

pub const NONE_RESOLVER: Option<&Resolver> = None;

pub trait ResolverExt: 'static {
    fn lookup_by_address<'a, P: IsA<InetAddress>, Q: IsA<Cancellable> + 'a, R: Into<Option<&'a Q>>>(&self, address: &P, cancellable: R) -> Result<GString, Error>;

    fn lookup_by_address_async<'a, P: IsA<InetAddress>, Q: IsA<Cancellable> + 'a, R: Into<Option<&'a Q>>, S: FnOnce(Result<GString, Error>) + Send + 'static>(&self, address: &P, cancellable: R, callback: S);

    #[cfg(feature = "futures")]
    fn lookup_by_address_async_future<P: IsA<InetAddress> + Clone + 'static, Q: IsA<Cancellable> + Clone + 'static>(&self, address: &P) -> Box_<futures_core::Future<Item = (Self, GString), Error = (Self, Error)>> where Self: Sized + Clone;

    fn lookup_by_name<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, hostname: &str, cancellable: Q) -> Result<Vec<InetAddress>, Error>;

    fn lookup_by_name_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<InetAddress>, Error>) + Send + 'static>(&self, hostname: &str, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn lookup_by_name_async_future<P: IsA<Cancellable> + Clone + 'static>(&self, hostname: &str) -> Box_<futures_core::Future<Item = (Self, Vec<InetAddress>), Error = (Self, Error)>> where Self: Sized + Clone;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: Q) -> Result<Vec<glib::Variant>, Error>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<glib::Variant>, Error>) + Send + 'static>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records_async_future<P: IsA<Cancellable> + Clone + 'static>(&self, rrname: &str, record_type: ResolverRecordType) -> Box_<futures_core::Future<Item = (Self, Vec<glib::Variant>), Error = (Self, Error)>> where Self: Sized + Clone;

    fn lookup_service<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, service: &str, protocol: &str, domain: &str, cancellable: Q) -> Result<Vec<SrvTarget>, Error>;

    fn lookup_service_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<SrvTarget>, Error>) + Send + 'static>(&self, service: &str, protocol: &str, domain: &str, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn lookup_service_async_future<P: IsA<Cancellable> + Clone + 'static>(&self, service: &str, protocol: &str, domain: &str) -> Box_<futures_core::Future<Item = (Self, Vec<SrvTarget>), Error = (Self, Error)>> where Self: Sized + Clone;

    fn set_default(&self);

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Resolver>> ResolverExt for O {
    fn lookup_by_address<'a, P: IsA<InetAddress>, Q: IsA<Cancellable> + 'a, R: Into<Option<&'a Q>>>(&self, address: &P, cancellable: R) -> Result<GString, Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_address(self.as_ref().to_glib_none().0, address.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_by_address_async<'a, P: IsA<InetAddress>, Q: IsA<Cancellable> + 'a, R: Into<Option<&'a Q>>, S: FnOnce(Result<GString, Error>) + Send + 'static>(&self, address: &P, cancellable: R, callback: S) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<S>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_by_address_async_trampoline<S: FnOnce(Result<GString, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_address_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<S>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_address_async_trampoline::<S>;
        unsafe {
            ffi::g_resolver_lookup_by_address_async(self.as_ref().to_glib_none().0, address.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn lookup_by_address_async_future<P: IsA<InetAddress> + Clone + 'static, Q: IsA<Cancellable> + Clone + 'static>(&self, address: &P) -> Box_<futures_core::Future<Item = (Self, GString), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let address = address.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.lookup_by_address_async(
                 &address,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn lookup_by_name<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, hostname: &str, cancellable: Q) -> Result<Vec<InetAddress>, Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name(self.as_ref().to_glib_none().0, hostname.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_by_name_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<InetAddress>, Error>) + Send + 'static>(&self, hostname: &str, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_by_name_async_trampoline<R: FnOnce(Result<Vec<InetAddress>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_name_async_trampoline::<R>;
        unsafe {
            ffi::g_resolver_lookup_by_name_async(self.as_ref().to_glib_none().0, hostname.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn lookup_by_name_async_future<P: IsA<Cancellable> + Clone + 'static>(&self, hostname: &str) -> Box_<futures_core::Future<Item = (Self, Vec<InetAddress>), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let hostname = String::from(hostname);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.lookup_by_name_async(
                 &hostname,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: Q) -> Result<Vec<glib::Variant>, Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_records(self.as_ref().to_glib_none().0, rrname.to_glib_none().0, record_type.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<glib::Variant>, Error>) + Send + 'static>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_records_async_trampoline<R: FnOnce(Result<Vec<glib::Variant>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_records_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_records_async_trampoline::<R>;
        unsafe {
            ffi::g_resolver_lookup_records_async(self.as_ref().to_glib_none().0, rrname.to_glib_none().0, record_type.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records_async_future<P: IsA<Cancellable> + Clone + 'static>(&self, rrname: &str, record_type: ResolverRecordType) -> Box_<futures_core::Future<Item = (Self, Vec<glib::Variant>), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let rrname = String::from(rrname);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.lookup_records_async(
                 &rrname,
                 record_type,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn lookup_service<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, service: &str, protocol: &str, domain: &str, cancellable: Q) -> Result<Vec<SrvTarget>, Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_service(self.as_ref().to_glib_none().0, service.to_glib_none().0, protocol.to_glib_none().0, domain.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_service_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<SrvTarget>, Error>) + Send + 'static>(&self, service: &str, protocol: &str, domain: &str, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_service_async_trampoline<R: FnOnce(Result<Vec<SrvTarget>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_service_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_service_async_trampoline::<R>;
        unsafe {
            ffi::g_resolver_lookup_service_async(self.as_ref().to_glib_none().0, service.to_glib_none().0, protocol.to_glib_none().0, domain.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn lookup_service_async_future<P: IsA<Cancellable> + Clone + 'static>(&self, service: &str, protocol: &str, domain: &str) -> Box_<futures_core::Future<Item = (Self, Vec<SrvTarget>), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let service = String::from(service);
        let protocol = String::from(protocol);
        let domain = String::from(domain);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.lookup_service_async(
                 &service,
                 &protocol,
                 &domain,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_resolver_set_default(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"reload\0".as_ptr() as *const _,
                transmute(reload_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn reload_trampoline<P>(this: *mut ffi::GResolver, f: glib_ffi::gpointer)
where P: IsA<Resolver> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Resolver::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resolver")
    }
}
