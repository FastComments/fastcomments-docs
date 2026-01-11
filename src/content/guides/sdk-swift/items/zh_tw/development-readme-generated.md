### 重新生成用戶端

若要根據最新的 OpenAPI 規格重新產生 API 用戶端：

1. 確保您已在本機於 `http://localhost:3001` 運行 FastComments 伺服器
2. 執行更新腳本：

```bash
./update.sh
```

此操作將會：
- 下載最新的 OpenAPI 規格
- 產生 Swift 用戶端程式碼（API 文件位於 client/docs）
- 建置套件以確認一切運作正常