FastCommentsでサイトからユーザーのコメント投稿を禁止する方法は2つあります。

1つ目は、既にメールアドレスを把握している場合、<a href="/auth/my-account/moderate-comments/banned-users" target="_blank">禁止ユーザー</a>ページに入力する方法です。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

このページは「コメントの管理」->「禁止ユーザー」からアクセスできます

ユーザーを禁止するとき、タイプは「永久」または「永久シャドウバン」のどちらかを選べます:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

ユーザーを禁止するもう一つの方法は、コメントの管理ページにある各コメントの禁止ボタンをクリックすることです。

禁止ボタンをクリックすると、禁止の種類と期間を指定できるオプションが表示されます。

### シャドウバン

シャドウバンは、ユーザーのコメントや投票が正常に保存されたように見せかけつつ、実際には保存されていない種類の禁止です。特定の状況ではこうした扱いが望ましい場合があります。

### IPアドレスによる禁止

テナントがオプトアウトを希望しない限り、FastCommentsは投稿者のIPアドレスのハッシュ化されたバージョンを保存することで、IPによる禁止をサポートしています。