use crate::types::*;

extern "C" {
    // HttpCurl
    #[link_name = "HttpCurl__new"]
    fn HttpCurl__new(this: *mut *mut ()) -> HttpCurlError;

    #[link_name = "HttpCurl__download"]
    fn HttpCurl__download(this: *const (), url: *const u8, url_len: usize, location: *const u8, location_len: usize) -> HttpCurlError;

    #[link_name = "HttpCurl__get"]
    fn HttpCurl__get(this: *const (), url: *const u8, url_len: usize, out: *mut CurlerString) -> HttpCurlError;
    
    #[link_name = "HttpCurl__progress_callback"]
    fn HttpCurl__progress_callback(this: *mut (), callback: extern "C" fn(*mut u8, f64, f64), user_data: *mut u8) -> HttpCurlError;

    // Drop for Curler
    #[link_name = "Curler__drop"]
    fn Curler__drop(curler: *mut ());
}

pub fn is_available() -> bool {
    if (HttpCurl__new as *const ()).is_null() {
        println!("Smashnet is not installed");
        false
    } else {
        true
    }
}

#[cfg(not(feature = "nro"))]
impl Curler {
    pub fn new() -> Result<Self, HttpCurlError> {
        unsafe {
            let mut this = std::ptr::null_mut();
            let error = HttpCurl__new(&mut this);
            match error {
                HttpCurlError::Ok => Ok(Self(this)),
                err => Err(err)
            }
        }
    }

    pub fn download(&self, url: String, location: String) -> Result<(), HttpCurlError> {
        unsafe {
            match HttpCurl__download(self.0, url.as_ptr(), url.len(), location.as_ptr(), location.len()) {
                HttpCurlError::Ok => Ok(()),
                err => Err(err)
            }
        }
    }

    pub fn get(&self, url: String) -> Result<String, HttpCurlError> {
        unsafe {
            let mut uninit = std::mem::MaybeUninit::uninit();
            match HttpCurl__get(self.0, url.as_ptr(), url.len(), uninit.as_mut_ptr()) {
                HttpCurlError::Ok => Ok(uninit.assume_init().into()),
                err => Err(err)
            }
        }
    }

    pub fn progress_callback<'a, F>(&mut self, callback: F) -> Result<&mut Self, HttpCurlError>
    where
        F: FnMut(f64, f64),
        F: 'a
    {
        extern "C" fn progress_callback(closure: *mut u8, downloaded: f64, total: f64) {
            let closure: &mut Box<dyn FnMut(f64, f64)> = unsafe { std::mem::transmute(closure) };
            (*closure)(downloaded, total)
        }
        unsafe {
            // hey man if you think this is shit you should see the rust std 
            let callback: Box<dyn FnMut(f64, f64) + 'static> = std::mem::transmute::<Box<dyn FnMut(f64, f64) + 'a>, Box<dyn FnMut(f64, f64) + 'static>>(Box::new(callback));
            let closure = Box::new(callback);
            let closure = Box::leak(closure);
            match HttpCurl__progress_callback(self.0, progress_callback, closure as *mut _ as *mut u8) {
                HttpCurlError::Ok => Ok(self),
                err => {
                    drop(Box::from_raw(closure as *mut Box<dyn FnMut(f64, f64)>));
                    Err(err)
                }
            }
        }
    }
}

#[cfg(not(feature = "not"))]
impl Drop for Curler {
    fn drop(&mut self) {
        unsafe {
            Curler__drop(self.0);
        }
    }
}