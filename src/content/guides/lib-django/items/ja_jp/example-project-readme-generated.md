実行可能なデモは[`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example)にあります: 左側レール + メインステージ  
ウィジェットごとにページがあるアプリで、**事前にシードされたデモユーザーを一覧表示するサインインページ**です。  
それらのいずれかでサインインすると、コメントとライブチャットウィジェットがその  
ID を **Secure SSO** で認証します。そのディレクトリから:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

APIシークレットがない場合、パブリックな `demo` テナント（匿名）にフォールバックします。  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) は Playwright の e2e  
実際のブラウザでページを読み込み、Secure-SSO  
ユーザーとしてコメントを投稿します。