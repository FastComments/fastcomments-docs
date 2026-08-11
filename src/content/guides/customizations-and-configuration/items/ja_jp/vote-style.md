[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

デフォルトでは、FastComments は投票オプションを上下矢印として表示し、ユーザーはコメントに対して賛成または反対の投票ができます。

ただし、投票ツールバーのスタイルを変更することが可能です。現在のオプションはデフォルトの上下ボタン、またはハートスタイルの投票メカニズムです。

**voteStyle** フラグは次のように使用します:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'ハートボタンを有効にする'; code-example-end]

コードなしで行うことを強くお勧めします。これによりサーバー側の検証も有効になります。ウィジェットカスタマイズページの「Vote Style」セクションをご覧ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='ウィジェットカスタマイズページの投票スタイル設定、上下矢印またはハート投票を提供'; title='投票スタイルを変更する' app-screenshot-end]

投票は無効にすることもできます。スタイルオプションの上にある `Disable Voting` を参照してください。