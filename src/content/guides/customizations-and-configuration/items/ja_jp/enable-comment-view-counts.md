[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments は各コメントを誰が閲覧したかを追跡せず、これに関する統計情報も提供しません。

ただし、この機能を有効にすると、システムは各ユーザーがコメントにスクロールしたときに追跡を開始します。

これが起こると、各コメントに表示される目のアイコンの横にあるカウントが増加します。カウントはリアルタイムで更新され、ユーザーのロケールに応じて省略形で表示されます。

この機能は **enableViewCounts** フラグを true に設定することで有効にできます：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'コメントビュー数の有効化'; code-example-end]

これはコードなしで、ウィジェットのカスタマイズページでカスタマイズできます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='ビュー数チェックボックスがオンになっているウィジェットカスタマイズページ。各コメントに目のアイコンとカウントが表示されます'; title='コメントビュー数の有効化' app-screenshot-end]

コメントを閲覧したユーザーID* を追跡します。これにより、同じコメントを再度閲覧してもカウントは増加しません。2 年後に再度閲覧した場合、カウントは増加します。

- *注: 匿名セッションID、またはハッシュ化されたユーザーのIPアドレスでも構いません。