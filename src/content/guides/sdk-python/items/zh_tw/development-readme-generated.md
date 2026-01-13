### 執行測試

```bash
# 設定環境變數
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# 執行測試
pytest tests/
```

### 重新生成用戶端

要從最新的 OpenAPI 規範重新生成 API 用戶端：

```bash
./update.sh
```

這將會：
1. 從正在執行的 FastComments 伺服器下載最新的 OpenAPI 規範（或使用本機的 openapi.yaml）
2. 生成 Python 用戶端程式碼
3. 扁平化目錄結構
4. 清理多餘的設定檔案