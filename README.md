为什么要用 rust ：
  - 在 solana 生态中 rust 是第一语言，使用同一种语言就可以开发公链、program(智能合约)、client。
  - 移动应用开发中使用 rust 可以无缝衔接，使用solana官方的工具库。

使用 rust lib 开发 solana 移动客户端，主要分为两部分：
  - rust lib 开发
  - 集成到 App （以 iOS 为例， Android 同样可以使用 rust lib）

代码已上传到 github： <https://github.com/kouliang/solana-ios.git>

---

### rust lib 开发
1. 创建 rust lib 项目
    cargo命令： `cargo new solana-core --lib`

2. 添加依赖库
  - solana-sdk  solana 开发中最基础的 SDK
  - solana-client 与 Solana 节点交互
  - rust-client  我自己封装的一个工具库

```
[lib]
name = "solana_core"
crate-type = ["staticlib"]

[dependencies]
libc = "0.2.169"
solana-client = "2.1.7"
solana-sdk = "2.1.7"
rust-client = { git = "https://github.com/kouliang/solana-client.git" }
```

3.方法实现
<br />
要遵循 rust 中 FFI 标准：  <https://doc.rust-lang.org/nomicon/ffi.html#rust-side>

test_key_pair 函数的功能是：接收外部传递过来的字符串，并尝试转换为 keypair 类型。如果转换成功则返回此keypair对应的账户地址，如果失败返回错误信息。

#[no_mangle]是必要的，告诉 Rust 编译器：不要乱改函数的名称。
```
#[no_mangle]
pub extern "C" fn test_key_pair(content: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(content) };
    let content = c_str.to_str().unwrap();

    let mut reader = Cursor::new(content.as_bytes());
    let keypair = keypair::read_keypair(&mut reader);

    match keypair {
        Ok(keypair) => {
            let pubkey = keypair.pubkey();
            let pubkey = pubkey.to_string();
            return std::ffi::CString::new(format!("Address: {:?}", pubkey)).unwrap().into_raw();
        },
        Err(e) => {
            return std::ffi::CString::new(format!("Error: {:?}", e)).unwrap().into_raw();
        }
    }
}

#[no_mangle]
pub extern "C" fn test_rpc(content: *const libc::c_char) -> *const libc::c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(content) };
    let content = c_str.to_str().unwrap();

    let lowercase = content.to_lowercase();

    let url = match lowercase.as_str() {
        "localhost" => "http://localhost:8899".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        other => other.to_string()
    };

    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let block_height = client.get_block_height();
    match block_height {
        Ok(block_height) => {
            return std::ffi::CString::new(format!("block_height: {:?}", block_height)).unwrap().into_raw();
        },
        Err(e) => {
            return std::ffi::CString::new(format!("Error: {:?}", e)).unwrap().into_raw();
        }
        
    }
}
```

4.交叉编译
  - 查看已安装和可用 targets 列表： `rustup target list`
  - 添加工具链： `rustup target add <your_target>`
  - 编译： `cargo build --target=<your_target> --release`

  target triple格式是 {arch}-{vendor}-{sys}-{abi}
  分别代表: 编译程序的主机系统、供应商、操作系统、ABI接口，最后abi有时候可以省略

  一定要根据开发环境选择对应的target才能编译出可用的库。我本地是 Appel M2 要在 模拟器上执行，所以选择的是 aarch64-apple-ios-sim

  最终的编译产物是 target/aarch64-apple-ios-sim/release/libsolana_core.a


---

### 集成到 App
1. 将 libsolana_core.a 添加到 iOS 项目中。具体操作不赘述，可以搜索 iOS添加静态库。

2. 创建桥接头文件
  - 在本项目中为 solana-Bridging-Header.h
  - 在 build settings 中链接该头文件：
    targets -> build settings -> swift compiler - general -> Objective-C Bridging Header -> YourProject-Bridging-Header.h

```
#ifndef Bridging_Header_h
#define Bridging_Header_h

#include <stdint.h>

const char* test_rpc(const char* content);
const char* test_key_pair(const char* content);

const char* save_config(const char* rpc, const char* keypair);

const char* balance(const char* content);
const char* transfer_to(const char* address, const char* amount);

#endif /* Bridging_Header_h */
```

3.swift 中直接使用 libsolana_core 中提供的方法。

```
@IBAction func balanceRequest(_ sender: Any) {
    let addr = balanceTextField.text ?? ""
    
    let result = balance(addr)!
    let resultstr = String(cString: result)
    print(resultstr)
    
    balanceLable.text = resultstr
}
@IBAction func transferRequest(_ sender: Any) {
    let addr = address.text ?? ""
    let am = amount.text ?? ""
    
    let result = transfer_to(addr, am)!
    let resultstr = String(cString: result)
    print(resultstr)
    
    transferLabel.text = resultstr
}
```

4.solana-ios App 使用说明
  - 首先要点击 Config 进入 Config 页，对 节点rpc_url 和 payer账户的keypair 进行配置。
  - 输入完成后点击 verify 进行验证。  rpc_url 配置正确下方会显示 "block_height: ..."， keypair 配置正确下方会显示"Address: ..."
  - 两项配置都验证通过后，点击右上角 "Save Config" 按钮会进行全局保存。
