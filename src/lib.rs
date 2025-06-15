use napi_derive::napi;
use napi::{Error, Result};
// use windows::core::*;
use windows::Win32::System::Com::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::Media::Audio::Endpoints::*;

// 初始化COM库
fn init_com() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|e| Error::from_reason(format!("COM 初始化失败: {e}")))
    }
}

// 暴露给 JS 的函数（返回字符串）
#[napi]
pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

// 获取麦克风录音灵敏度 (0.0-1.0)
#[napi]
pub fn get_microphone_sensitivity() -> Result<f32>  {
    init_com()?;
    // 创建设备枚举器实例
    let enumerator: IMMDeviceEnumerator = unsafe {
        CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_INPROC_SERVER
        ).map_err(|e| Error::from_reason(format!("创建设备枚举器实例失败: {e}")))?
    };
    // 打印设备列表
    println!("设备列表：{:?}", enumerator);
    // 获取默认输入设备
    let device: IMMDevice = unsafe {
        enumerator.GetDefaultAudioEndpoint(
            eCapture,
            eMultimedia
        ).map_err(|e| Error::from_reason(format!("获取默认音频设备失败: {e}")))?
    };
    println!("默认输入设备：{:?}", device);

    // 判断设备状态
    let state: DEVICE_STATE = unsafe {
        device.GetState().map_err(|e| Error::from_reason(format!("获取设备状态失败: {e}")))?
    };

    println!("设备状态：{:?}", state);

    if state != DEVICE_STATE_ACTIVE {
        return Err(Error::from_reason("当前默认麦克风未插入或不可用"));
    }

    // 激活音量控制接口
    let endpoint: IAudioEndpointVolume = unsafe {
    device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)
        .map_err(|e| Error::from_reason(format!("激活音量接口失败: {e}")))?
    };
    println!("音量接口：{:?}", endpoint);

    let volume: f32 = unsafe {
        endpoint.GetMasterVolumeLevelScalar().map_err(|e| Error::from_reason(format!("获取音量失败: {e}")))?
    };
    println!("当前音量：{}", volume);

    // unsafe { 
    //     CoUninitialize()
    // };
    return Ok(volume);
}
