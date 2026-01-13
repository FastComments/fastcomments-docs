[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastCommentsはユーザーが他のユーザーをブロックできるようになっています。ブロックされたユーザーのコメントはマスクされ、
ユーザー間の通知が無効になり、その他の影響があります。

この機能を無効化したい場合があります。次のように行えます：

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

これはコードを使わずに、ウィジェットカスタマイズUIを通じて行うこともでき、その場合はサーバー側での適切な検証も有効になります：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---