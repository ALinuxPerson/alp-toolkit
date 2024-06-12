use std::env;
use std::env::VarError;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use thiserror::Error;

#[macro_export]
macro_rules! location {
    (
        $mod_vis:vis mod $module_name:ident;
        $env_prefix:literal;

        $($fn_name:ident = $fn_expr:expr;)*
    ) => {
        $mod_vis mod $module_name {
            static PROVIDER: ::std::sync::OnceLock<$crate::ProjectDirsOrEnv> = ::std::sync::OnceLock::new();
            
            pub fn initialize() -> ::core::result::Result<(), $crate::InitializeError> {
                if PROVIDER.set($crate::ProjectDirsOrEnv::new(env!("CARGO_PKG_NAME"), $env_prefix)?).is_err() {
                    panic!("project directories/env provider already initialized")
                }
                
                Ok(())
            }
            
            fn provider() -> &'static $crate::ProjectDirsOrEnv {
                PROVIDER.get().expect("project directories/env provider not yet initialized")
            }
            
            pub fn cache_dir() -> &'static ::std::path::Path {
                provider().cache_dir()
            }
            
            pub fn config_dir() -> &'static ::std::path::Path {
                provider().config_dir()
            }
            
             pub fn config_local_dir() -> &'static ::std::path::Path {
                provider().config_local_dir()
            }
            
            pub fn data_dir() -> &'static ::std::path::Path {
                provider().data_dir()
            }
            
            pub fn data_local_dir() -> &'static ::std::path::Path {
                provider().data_local_dir()
            }           
            
            pub fn preference_dir() -> &'static ::std::path::Path {
                provider().preference_dir()
            }
            
            pub fn project_path() -> &'static ::std::path::Path {
                provider().project_path()
            }
            
            pub fn runtime_dir() -> Option<&'static ::std::path::Path> {
                provider().runtime_dir()
            }

            pub fn state_dir() -> Option<&'static ::std::path::Path> {
                provider().state_dir()
            }
            
            $(pub fn $fn_name() -> &'static ::std::path::Path {
                static VALUE: ::std::sync::OnceLock<::std::path::PathBuf> = ::std::sync::OnceLock::new();
                
                VALUE.get_or_init(|| {
                    let f: fn(&'static $crate::ProjectDirsOrEnv) -> ::std::path::PathBuf = $fn_expr;
                    
                    f(provider())
                })
            })*
        }
    };
}

trait Provider: Sized {
    type Init<'a>;
    type Error;

    fn new(init: Self::Init<'_>) -> Result<Self, Self::Error>;
    fn cache_dir(&self) -> Option<&Path>;
    fn config_dir(&self) -> Option<&Path>;
    fn config_local_dir(&self) -> Option<&Path>;
    fn data_dir(&self) -> Option<&Path>;
    fn data_local_dir(&self) -> Option<&Path>;
    fn preference_dir(&self) -> Option<&Path>;
    fn project_path(&self) -> Option<&Path>;
    fn runtime_dir(&self) -> Option<&Path>;
    fn state_dir(&self) -> Option<&Path>;
}

#[derive(Error, Debug)]
#[error("the home directory could not be found")]
#[non_exhaustive]
pub struct HomeDirNotFoundError;

impl Provider for ProjectDirs {
    type Init<'a> = &'a str;
    type Error = HomeDirNotFoundError;

    fn new(app_name: Self::Init<'_>) -> Result<Self, Self::Error> {
        Self::from("", "ALinuxPerson", app_name).ok_or(HomeDirNotFoundError)
    }

    fn cache_dir(&self) -> Option<&Path> {
        Some(self.cache_dir())
    }

    fn config_dir(&self) -> Option<&Path> {
        Some(self.config_dir())
    }

    fn config_local_dir(&self) -> Option<&Path> {
        Some(self.config_local_dir())
    }

    fn data_dir(&self) -> Option<&Path> {
        Some(self.data_dir())
    }

    fn data_local_dir(&self) -> Option<&Path> {
        Some(self.data_local_dir())
    }

    fn preference_dir(&self) -> Option<&Path> {
        Some(self.preference_dir())
    }

    fn project_path(&self) -> Option<&Path> {
        Some(self.project_path())
    }

    fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir()
    }

    fn state_dir(&self) -> Option<&Path> {
        self.state_dir()
    }
}

struct Env {
    cache_dir: Option<PathBuf>,
    config_dir: Option<PathBuf>,
    config_local_dir: Option<PathBuf>,
    data_dir: Option<PathBuf>,
    data_local_dir: Option<PathBuf>,
    preference_dir: Option<PathBuf>,
    project_path: Option<PathBuf>,
    runtime_dir: Option<PathBuf>,
    state_dir: Option<PathBuf>,
}

#[derive(Error, Debug)]
#[error("the environment variable '{name}' does not contain valid unicode data")]
#[non_exhaustive]
pub struct EnvVarNotUnicodeError {
    pub name: String,
    pub value: OsString,
}

impl Provider for Env {
    type Init<'a> = &'a str;
    type Error = EnvVarNotUnicodeError;

    fn new(env_prefix: Self::Init<'_>) -> Result<Self, Self::Error> {
        let x = |suffix: &str| {
            let key = format!("{env_prefix}{suffix}");

            match env::var(&key) {
                Ok(value) => Ok(Some(PathBuf::from(value))),
                Err(VarError::NotPresent) => Ok(None),
                Err(VarError::NotUnicode(value)) => Err(EnvVarNotUnicodeError { name: key, value }),
            }
        };

        Ok(Self {
            cache_dir: x("_CACHE_DIR")?,
            config_dir: x("_CONFIG_DIR")?,
            config_local_dir: x("_CONFIG_LOCAL_DIR")?,
            data_dir: x("_DATA_DIR")?,
            data_local_dir: x("_DATA_LOCAL_DIR")?,
            preference_dir: x("_PREFERENCE_DIR")?,
            project_path: x("_PROJECT_PATH")?,
            runtime_dir: x("_RUNTIME_DIR")?,
            state_dir: x("_STATE_DIR")?,
        })
    }

    fn cache_dir(&self) -> Option<&Path> {
        self.cache_dir.as_deref()
    }

    fn config_dir(&self) -> Option<&Path> {
        self.config_dir.as_deref()
    }

    fn config_local_dir(&self) -> Option<&Path> {
        self.config_local_dir.as_deref()
    }

    fn data_dir(&self) -> Option<&Path> {
        self.data_dir.as_deref()
    }

    fn data_local_dir(&self) -> Option<&Path> {
        self.data_local_dir.as_deref()
    }

    fn preference_dir(&self) -> Option<&Path> {
        self.preference_dir.as_deref()
    }

    fn project_path(&self) -> Option<&Path> {
        self.project_path.as_deref()
    }

    fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_deref()
    }

    fn state_dir(&self) -> Option<&Path> {
        self.state_dir.as_deref()
    }
}

impl Env {
    fn _parity(&mut self) -> Option<EnvParity> {
        if self.cache_dir.is_none() || self.config_dir.is_none() || self.config_dir.is_none() 
            || self.config_local_dir.is_none() || self.data_dir.is_none() 
            || self.data_local_dir.is_none() || self.preference_dir.is_none() 
            || self.project_path.is_none() {
            return None
        }
        
        Some(EnvParity {
            cache_dir: self.cache_dir.take()?,
            config_dir: self.config_dir.take()?,
            config_local_dir: self.config_local_dir.take()?,
            data_dir: self.data_dir.take()?,
            data_local_dir: self.data_local_dir.take()?,
            preference_dir: self.preference_dir.take()?,
            project_path: self.project_path.take()?,
            runtime_dir: self.runtime_dir.take(),
            state_dir: self.state_dir.take(),
        })       
    }
   
    #[allow(clippy::result_large_err)]
    fn parity(mut self) -> Result<EnvParity, Self> {
        match self._parity() {
            Some(parity) => Ok(parity),
            None => Err(self),
        }
    }
}

struct EnvParity {
    cache_dir:        PathBuf,
    config_dir:       PathBuf,
    config_local_dir: PathBuf,
    data_dir:         PathBuf,
    data_local_dir:   PathBuf,
    preference_dir:   PathBuf,
    project_path:     PathBuf,
    runtime_dir:      Option<PathBuf>,
    state_dir:        Option<PathBuf>,
}

#[derive(Error, Debug)]
pub enum InitializeError {
    #[error("project dirs provider failed to initialize")]
    ProjectDirs(#[from] HomeDirNotFoundError),

    #[error("env provider failed to initialize")]
    Env(#[from] EnvVarNotUnicodeError),
}

pub struct ProjectDirsOrEnv {
    cache_dir:        PathBuf,
    config_dir:       PathBuf,
    config_local_dir: PathBuf,
    data_dir:         PathBuf,
    data_local_dir:   PathBuf,
    preference_dir:   PathBuf,
    project_path:     PathBuf,
    runtime_dir:      Option<PathBuf>,
    state_dir:        Option<PathBuf>,
}

impl From<EnvParity> for ProjectDirsOrEnv {
    fn from(value: EnvParity) -> Self {
        Self {
            cache_dir: value.cache_dir,
            config_dir: value.config_dir,
            config_local_dir: value.config_local_dir,
            data_dir: value.data_dir,
            data_local_dir: value.data_local_dir,
            preference_dir: value.preference_dir,
            project_path: value.project_path,
            runtime_dir: value.runtime_dir,
            state_dir: value.state_dir,
        }
    }
}

impl ProjectDirsOrEnv {
    pub fn new(app_name: &str, env_prefix: &str) -> Result<Self, InitializeError> {
        let env = Env::new(env_prefix)?;
        
        match env.parity().map(Self::from) {
            Ok(this) => Ok(this),
            Err(env) => {
                let project_dirs = ProjectDirs::new(app_name)?;

                Ok(Self {
                    cache_dir: env.cache_dir.unwrap_or(PathBuf::from(project_dirs.cache_dir())),
                    config_dir: env.config_dir.unwrap_or(PathBuf::from(project_dirs.config_dir())),
                    config_local_dir: env.config_local_dir.unwrap_or(PathBuf::from(project_dirs.config_local_dir())),
                    data_dir: env.data_dir.unwrap_or(PathBuf::from(project_dirs.data_dir())),
                    data_local_dir: env.data_local_dir.unwrap_or(PathBuf::from(project_dirs.data_local_dir())),
                    preference_dir: env.preference_dir.unwrap_or(PathBuf::from(project_dirs.preference_dir())),
                    project_path: env.project_path.unwrap_or(PathBuf::from(project_dirs.project_path())),
                    runtime_dir: None,
                    state_dir: None,
                })               
            }
        }
    }
}

impl ProjectDirsOrEnv {
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
    
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
    
    pub fn config_local_dir(&self) -> &Path {
        &self.config_local_dir
    }
    
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
    
    pub fn data_local_dir(&self) -> &Path {
        &self.data_local_dir
    }
    
    pub fn preference_dir(&self) -> &Path {
        &self.preference_dir
    }
    
    pub fn project_path(&self) -> &Path {
        &self.project_path
    }
    
    pub fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_deref()
    }

    pub fn state_dir(&self) -> Option<&Path> {
        self.state_dir.as_deref()
    }
}
