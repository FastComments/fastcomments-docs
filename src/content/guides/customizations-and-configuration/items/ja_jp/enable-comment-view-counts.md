[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments は各コメントを誰が閲覧したかを追跡したり、その統計を提供したりしません。

しかし、この機能を有効にすると、ユーザーがコメントまでスクロールするたびにシステムがその閲覧を追跡し始めます。

これが起きると、各コメントに表示される目のアイコンの横にあるカウントが増加します。カウントはリアルタイムで更新され、ユーザーのロケールに応じて省略表示されます。

この機能は **enableViewCounts** フラグを true に設定することで有効にできます:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

これはウィジェットカスタマイズページで、コードを使わずに設定できます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

コメントを閲覧した user id* を追跡するため、同じユーザーがコメントを再度閲覧してもカウントは増えません。2年後に再度そのコメントを閲覧した場合、カウントは再び増加します。

- *注: または anon session id、あるいはユーザーの IP をハッシュ化した値。