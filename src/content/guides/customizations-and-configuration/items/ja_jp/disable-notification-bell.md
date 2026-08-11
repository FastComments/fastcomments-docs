[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメント領域の右上に通知ベルを表示します。

このベルは赤くなり、ユーザーが受け取っている通知の数を表示します。例として以下の通知があります：

- ユーザーがあなたに返信しました。
- ユーザーがあなたがコメントしたスレッドで返信しました。
- ユーザーがあなたのコメントに賛成票を付けました。
- ユーザーがあなたが購読しているページに返信しました。

通知ベルは、ページ全体を購読する機能も提供します。

ただし、通知ベルを完全に無効にすることもできます：

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

コードを使用せずにこれを行うこともできます。ウィジェットカスタマイズページで「Disable Notification Bell」セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='通知ベル無効化チェックボックスがチェックされたウィジェットカスタマイズページ'; title='通知ベルを無効にする' app-screenshot-end]