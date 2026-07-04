---
要自动将已登录用户传递给小部件，标签会从请求中读取当前用户。确保你的项目具备以下两项（在标准 Django 项目中默认启用）：

- `django.template.context_processors.request` 在 `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` 在 `MIDDLEWARE`

如果模板上下文中没有请求，组件将为匿名访客渲染。你也可以显式传递用户：`{% fastcomments user=some_user %}`。
---