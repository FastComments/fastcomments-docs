FastCommentsでサイトでのコメントを禁止する方法は2つあります。

最初の方法は、既にメールアドレスが分かっている場合に、<a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">禁止ユーザー</a>ページに入力することです。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

このページには Moderate Comments -> Banned Users からアクセスできます

ユーザーを禁止する際、Permanent または Permanent Shadow Ban のいずれかを選択できます:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

2つ目の方法は、Comment Moderation ページの各コメントに配置されている ban ボタンをクリックすることです。

ban ボタンをクリックすると、禁止タイプと期間を指定できるオプションが表示されます。

### メールエイリアス

メールでユーザーを禁止する場合、FastComments は自動的に `+` エイリアスを無視します。例えば、`user+alias@gmail.com` を禁止すると、`user@gmail.com` や `user+other@gmail.com` のようなそのアドレスの他の `+` バリエーションも禁止されます。

### シャドウバン

シャドウバンは、ユーザーのコメントや投票が正常に保存されたように見せかけるタイプの禁止ですが、実際には保存されていません。特定の状況ではこれが望ましい場合があります。

### IPアドレスによる禁止

テナントがオプトアウトしない限り、FastComments は投稿者の IP アドレスのハッシュ化されたバージョンを保存することで、IP による禁止をサポートします。