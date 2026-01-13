デフォルトでは、各ユーザーは同じ1分間に最大`5 comments`を投稿できます。

これは user id、anon user id、および ip address (hashed) によって追跡されます。

これはウィジェットのカスタマイズページでコード不要でカスタマイズできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

注意: comment creation API を使用している場合、リクエストでユーザーの元の `ip` アドレスを当社のバックエンドに渡すことを検討してください。レート制限がユーザーごとに適用され、アカウント全体には適用されないようにするためです。