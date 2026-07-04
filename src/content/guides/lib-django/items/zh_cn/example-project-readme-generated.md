---
可运行的演示位于 [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example)：一个 left-rail + main-stage  
应用，每个部件都有一个页面，并且包含一个**列出预先种子演示用户的登录页面**。  
使用其中任意用户登录，评论和实时聊天部件将通过**Secure SSO** 验证该身份。  
在该目录下运行：

```bash
python manage.py migrate
# 使用您自己的租户以查看 Secure SSO 的实际效果（需要 API secret）：
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

如果没有 API secret，则会回退到公共 `demo` 租户（匿名）。  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) 是一个 Playwright e2e  
它在真实浏览器中加载页面并以 Secure-SSO 用户身份发布评论。