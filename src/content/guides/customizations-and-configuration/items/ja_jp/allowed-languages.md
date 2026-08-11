---
デフォルトでは、FastComments はコメントに使用される言語を制限しません。 

コミュニティが使用する言語を制限したい場合があります。

この設定はコードを書かずに、ウィジェットカスタマイズページで行えます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='コメントで使用できる言語を制限するためのウィジェットカスタマイズページ上の許可された言語セレクタ'; title='許可された言語' app-screenshot-end]

システムはコメントを解析して言語を判定し、許可リストと照合します。

コメントが許可されていない言語で書かれている場合、ローカライズされたエラーメッセージが表示されます。 

---