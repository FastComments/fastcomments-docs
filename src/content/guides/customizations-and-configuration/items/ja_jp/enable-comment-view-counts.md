[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments は各コメントを誰が閲覧したかを追跡せず、これに関する統計情報も提供しません。

ただし、この機能を有効にすれば、システムは各ユーザーがコメントにスクロールした際に追跡を開始します。

このようになると、各コメントに表示される目のアイコンの横にあるカウントが増加します。カウントはリアルタイムで更新され、ユーザーのロケールに合わせて省略形で表示されます。

**enableViewCounts** フラグを true に設定することで、この機能を有効にできます：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'コメント閲覧数の有効化'; code-example-end]

コードを書かずに、ウィジェットのカスタマイズページで設定できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='ビュー数チェックボックスがオンになっているウィジェットカスタマイズページ。各コメントに目のアイコンとカウントが表示されます'; title='コメント閲覧数の有効化' app-screenshot-end]

コメントを閲覧したユーザーID* を1週間追跡します。そのため、同じ週内に再度コメントを閲覧してもカウントは増加しません。1週間が経過した後に再度閲覧すると、カウントは再び増加します。

- *注: 匿名セッションID、またはユーザーのIPをハッシュ化した値でも構いません。