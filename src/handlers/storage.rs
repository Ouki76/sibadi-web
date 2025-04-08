use web_sys::window;

pub trait Storage {
    fn get(key: impl AsRef<str>) -> Option<String>;
    fn set<T>(key: T, value: T) where T: AsRef<str>;
    fn remove(key: impl AsRef<str>);
}

pub struct Local;

impl Storage for Local {
    fn get(key: impl AsRef<str>) -> Option<String> {
        window()
            .and_then(|w| w.local_storage().ok()?)
            .and_then(|s| s.get_item(key.as_ref()).ok().flatten())
    }

    fn set<T>(key: T, value: T) 
        where 
            T: AsRef<str> 
    {
        if let Some(s) = window()
            .and_then(|w| w.local_storage().ok().flatten())
        {
            let _ = s.set_item(key.as_ref(), value.as_ref());
        }
    }

    fn remove(key: impl AsRef<str>) {
        if let Some(s) = window()
            .and_then(|w| w.local_storage().ok().flatten())
        {
            let _ = s.remove_item(key.as_ref());
        }
    }
}

pub struct Session;

impl Storage for Session {
    fn get(key: impl AsRef<str>) -> Option<String> {
        window()
            .and_then(|w| w.session_storage().ok()?)
            .and_then(|s| s.get_item(key.as_ref()).ok().flatten())
    }

    fn set<T>(key: T, value: T) 
        where 
            T: AsRef<str> 
    {
        if let Some(s) = window()
            .and_then(|w| w.session_storage().ok().flatten())
        {
            let _ = s.set_item(key.as_ref(), value.as_ref());
        }
    }

    fn remove(key: impl AsRef<str>) {
        if let Some(s) = window()
            .and_then(|w| w.session_storage().ok().flatten())
        {
            let _ = s.remove_item(key.as_ref());
        }
    }
}
