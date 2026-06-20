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

此函式庫包含產生的 API 用戶端以及讓操作 API 更簡便的 SSO 工具。

- [API 用戶端函式庫文件](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公開與受保護的 APIs

對於 API 用戶端，有三個類別，`DefaultApi`、`PublicApi` 與 `ModerationApi`。 `DefaultApi` 包含需要您的 API 金鑰的方法，而 `PublicApi` 包含
可以直接從瀏覽器/行動裝置/等執行且不需驗證的方法。 `ModerationApi` 包含驅動管理者儀表板的方法 - 列出、
計數、搜尋、匯出與擷取評論的日誌、審核動作（移除/還原、標記、設定審查/垃圾/批准狀態、調整投票、重新開啟/關閉討論串）、
禁令（從評論中封禁、撤銷禁令、預先封禁摘要、封禁狀態與偏好、被封禁使用者計數）、以及徽章與信任（授予/移除徽章、手動徽章、取得/設定信任
係數、使用者內部資料）。每個 `ModerationApi` 方法都接受一個 `sso` 參數，因此呼叫會以經 SSO 驗證的管理者身分執行。