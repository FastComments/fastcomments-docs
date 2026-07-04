---
一個可執行的示範位於 [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example)：一個左側導覽列 + 主舞台的
應用程式，為每個 widget 提供一個頁面，並有 **列出預先種子化示範使用者的登入頁面**。  
使用其中任何一個帳號登入，評論與即時聊天 widget 會透過 **Secure SSO** 驗證該身分。從該目錄：

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

如果未提供 API 金鑰，則會回退到公共的 `demo` 租戶（匿名）。  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) 是一個 Playwright e2e
測試，會在真實瀏覽器中載入頁面，並以 Secure-SSO 使用者身分發表評論。
---