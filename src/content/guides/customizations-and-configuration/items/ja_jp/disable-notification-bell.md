[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメントエリアの右上に通知ベルを表示します。

このベルは赤くなり、ユーザーが持っている通知の数を表示します。通知の例は次のとおりです：

- ユーザーがあなたに返信しました。
- ユーザーがあなたがコメントしたスレッドに返信しました。
- ユーザーがあなたのコメントに賛成票を投じました。
- ユーザーがあなたが購読しているページに返信しました。

通知ベルは、ページ全体を購読するための仕組みも提供します。

ただし、通知ベルを完全に無効にすることもできます：

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

コードを書かなくてもこれを行うことができます。ウィジェットのカスタマイズページで「通知ベルを無効にする」セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]