// Bindings generated by `windows-bindgen` 0.56.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_core::imp::define_interface!(
    IDeviceService,
    IDeviceService_Vtbl,
    0x650dd97b_8903_5e67_9156_000f731209e0
);
impl windows_core::RuntimeType for IDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        std::mem::MaybeUninit<windows_core::HSTRING>,
        *mut std::mem::MaybeUninit<DeviceData>,
    ) -> windows_core::HRESULT,
    pub CreateRandomDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut std::mem::MaybeUninit<DeviceData>,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDeviceServiceFactory,
    IDeviceServiceFactory_Vtbl,
    0x60e934a8_1152_5f21_a40e_52d4f67023f9
);
impl windows_core::RuntimeType for IDeviceServiceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceServiceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDeviceStore,
    IDeviceStore_Vtbl,
    0x17a36390_c788_59cb_b3a9_113bb13898a2
);
impl std::ops::Deref for IDeviceStore {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDeviceStore,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IDeviceStore {
    pub fn Load(&self, deviceid: i32) -> windows_core::Result<DeviceData> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Load)(
                windows_core::Interface::as_raw(this),
                deviceid,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Save<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DeviceData>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Save)(
                windows_core::Interface::as_raw(this),
                data.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for IDeviceStore {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut std::mem::MaybeUninit<DeviceData>,
    ) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        std::mem::MaybeUninit<DeviceData>,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DeviceService,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DeviceService {
    pub fn CreateDevice(
        &self,
        deviceid: i32,
        description: &windows_core::HSTRING,
    ) -> windows_core::Result<DeviceData> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDevice)(
                windows_core::Interface::as_raw(this),
                deviceid,
                core::mem::transmute_copy(description),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateRandomDevice(&self) -> windows_core::Result<DeviceData> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRandomDevice)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0>(store: P0) -> windows_core::Result<DeviceService>
    where
        P0: windows_core::Param<IDeviceStore>,
    {
        Self::IDeviceServiceFactory(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                store.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeviceServiceFactory<
        R,
        F: FnOnce(&IDeviceServiceFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceService, IDeviceServiceFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDeviceService>();
}
unsafe impl windows_core::Interface for DeviceService {
    type Vtable = IDeviceService_Vtbl;
    const IID: windows_core::GUID = <IDeviceService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceService {
    const NAME: &'static str = "win_example.DeviceService";
}
unsafe impl Send for DeviceService {}
unsafe impl Sync for DeviceService {}
#[repr(C)]
pub struct DeviceData {
    pub DeviceId: i32,
    pub Description: windows_core::HSTRING,
}
impl Clone for DeviceData {
    fn clone(&self) -> Self {
        Self {
            DeviceId: self.DeviceId,
            Description: self.Description.clone(),
        }
    }
}
impl core::fmt::Debug for DeviceData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DeviceData")
            .field("DeviceId", &self.DeviceId)
            .field("Description", &self.Description)
            .finish()
    }
}
impl windows_core::TypeKind for DeviceData {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for DeviceData {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(win_example.DeviceData;i4;string)");
}
impl PartialEq for DeviceData {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.Description == other.Description
    }
}
impl Eq for DeviceData {}
impl Default for DeviceData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub trait IDeviceService_Impl: Sized {
    fn CreateDevice(
        &self,
        deviceid: i32,
        description: &windows_core::HSTRING,
    ) -> windows_core::Result<DeviceData>;
    fn CreateRandomDevice(&self) -> windows_core::Result<DeviceData>;
}
impl windows_core::RuntimeName for IDeviceService {
    const NAME: &'static str = "win_example.IDeviceService";
}
impl IDeviceService_Vtbl {
    pub const fn new<
        Identity: windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDeviceService_Impl,
        const OFFSET: isize,
    >() -> IDeviceService_Vtbl {
        unsafe extern "system" fn CreateDevice<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDeviceService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            deviceid: i32,
            description: std::mem::MaybeUninit<windows_core::HSTRING>,
            result__: *mut std::mem::MaybeUninit<DeviceData>,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceService_Impl::CreateDevice(
                this,
                deviceid,
                core::mem::transmute(&description),
            ) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRandomDevice<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDeviceService_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut std::mem::MaybeUninit<DeviceData>,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceService_Impl::CreateRandomDevice(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceService, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            CreateRandomDevice: CreateRandomDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceService as windows_core::Interface>::IID
    }
}
pub trait IDeviceServiceFactory_Impl: Sized {
    fn CreateInstance(&self, store: Option<&IDeviceStore>) -> windows_core::Result<DeviceService>;
}
impl windows_core::RuntimeName for IDeviceServiceFactory {
    const NAME: &'static str = "win_example.IDeviceServiceFactory";
}
impl IDeviceServiceFactory_Vtbl {
    pub const fn new<
        Identity: windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDeviceServiceFactory_Impl,
        const OFFSET: isize,
    >() -> IDeviceServiceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDeviceServiceFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            store: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceServiceFactory_Impl::CreateInstance(
                this,
                windows_core::from_raw_borrowed(&store),
            ) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceServiceFactory, OFFSET>(
            ),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceServiceFactory as windows_core::Interface>::IID
    }
}
pub trait IDeviceStore_Impl: Sized {
    fn Load(&self, deviceid: i32) -> windows_core::Result<DeviceData>;
    fn Save(&self, data: &DeviceData) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDeviceStore {
    const NAME: &'static str = "win_example.IDeviceStore";
}
impl IDeviceStore_Vtbl {
    pub const fn new<
        Identity: windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDeviceStore_Impl,
        const OFFSET: isize,
    >() -> IDeviceStore_Vtbl {
        unsafe extern "system" fn Load<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDeviceStore_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            deviceid: i32,
            result__: *mut std::mem::MaybeUninit<DeviceData>,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceStore_Impl::Load(this, deviceid) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDeviceStore_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            data: std::mem::MaybeUninit<DeviceData>,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceStore_Impl::Save(this, core::mem::transmute(&data)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceStore, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceStore as windows_core::Interface>::IID
    }
}
