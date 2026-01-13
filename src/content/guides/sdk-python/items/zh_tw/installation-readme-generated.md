### PyPI

```bash
pip install fastcomments
```

### 函式庫內容

此函式庫包含兩個模組：生成的 API 客戶端與核心 Python 函式庫，後者包含手寫的工具以便更容易使用 API，包括 SSO 支援。

- [API 客戶端函式庫文件](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心函式庫文件（含 SSO 範例）](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開與受保護的 API

對於 API 客戶端，有兩個類別，`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要您的 API 金鑰 的方法，而 `PublicApi` 包含可以直接從瀏覽器、行動裝置等在未經驗證的情況下呼叫的 API。