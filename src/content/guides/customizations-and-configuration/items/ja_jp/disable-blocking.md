[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーが他のユーザーをブロックできるようにしています。ユーザーをブロックすると、そのコメントが  
マスクされ、ユーザー間の通知が防止されるなどの効果があります。

この機能を無効にしたい場合があります。以下のように実行できます：

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'ブロックの無効化'; code-example-end]

コードを使用せずに、ウィジェットカスタマイズ UI を使用して実行することもでき、これにより適切なサーバー側検証も有効になります：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='ウィジェットカスタマイズ UI のブロック無効化オプション。ユーザーが互いにブロックするのを防止します'; title='ブロックの無効化' app-screenshot-end]