FastCommentsでサイトのコメントを禁止する方法は2つあります。

1つ目は、既にメールアドレスが分かっている場合、<a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">禁止ユーザー</a>ページに入力する方法です。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

このページには Moderate Comments -> Banned Users からアクセスできます

ユーザーを禁止する際、種類として Permanent または Permanent Shadow Ban を選択できます:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

2つ目の方法は、Comment Moderation ページの各コメントに配置された禁止ボタンをクリックすることです。

禁止ボタンをクリックすると、禁止の種類や期間を指定できるオプションが表示されます。

### シャドウバン

シャドウバンは、ユーザーのコメントや投票が正常に保存されたように見せかける一方で、実際には保存されていない種類の禁止です。特定の状況では望ましい場合があります。

### IPアドレスによる禁止

テナントがオプトアウトしない限り、FastComments はコメント投稿者のIPアドレスのハッシュ化したバージョンを保存することで、IPによる禁止をサポートしています。