デフォルトでは、FastCommentsはコメントに使用される言語を制限しません。 

コミュニティで使用する言語を制限したい場合があります。

これはコード不要で、ウィジェットのカスタマイズページで設定できます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

システムはコメントを解析してその言語を判定し、許可リストと照合します。

コメントが許可されていない言語で書かれている場合は、ローカライズされたエラーメッセージが表示されます。