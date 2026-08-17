---
ログインしたユーザーをウィジェットに自動的に渡すには、タグはリクエストから現在のユーザーを  
取得します。プロジェクトに以下の2つが設定されていることを確認してください（  
標準的な Django プロジェクトではデフォルトで有効になっています）:

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

テンプレートコンテキストにリクエストが無い場合、ウィジェットは匿名の  
訪問者向けにレンダリングされます。常にユーザーを明示的に渡すこともできます: `{% fastcomments user=some_user %}`.
---