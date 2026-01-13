### 安裝相依套件

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### 從原始碼建置

```bash
mkdir build
cd build
cmake ..
make
```

### 安裝

```bash
sudo make install
```

### 函式庫內容

此函式庫包含產生的 API 用戶端以及 SSO 工具，讓使用 API 更加方便。

- [API 用戶端函式庫文件](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公開與受保護的 API

對於 API 用戶端，有兩個類別，`DefaultAPI` 與 `PublicAPI`。`DefaultAPI` 包含需要使用你的 API 金鑰的方法，而 `PublicAPI` 則包含可直接從瀏覽器/行動裝置/等在不需驗證的情況下呼叫的 API。