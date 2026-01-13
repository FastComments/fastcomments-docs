[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

デフォルトでは、FastComments は投票オプションを上矢印と下矢印として表示し、ユーザーはコメントに対して賛成（アップ）または反対（ダウン）投票を行えます。

ただし、投票ツールバーのスタイルを変更することができます。現在のオプションは、デフォルトの上下ボタン、またはハートスタイルの投票メカニズムです。

以下のように **voteStyle** フラグを使用します：

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

サーバー側の検証も有効になるため、コードを使わずに行うことを強く推奨します。ウィジェットのカスタマイズページで "Vote Style" セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

投票は無効にすることもできます。スタイルオプションの上にある `Disable Voting` を参照してください。

---