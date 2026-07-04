---
若要自動將已登入的使用者傳遞給小工具，標籤會從請求中讀取當前使用者。請確保您的專案具備以下兩者（在標準的 Django 專案中預設已啟用）：

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

如果模板上下文中沒有請求，則小工具會以匿名訪客的身份呈現。您也可以隨時明確傳遞使用者：`{% fastcomments user=some_user %}`。
---