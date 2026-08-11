---
[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

readonly フラグを true に設定することで、コメント機能をロックし、新しいコメントや投票を受け付けなくすることができます。

コメントは編集や削除もできなくなります。

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'コメントスレッドを読み取り専用にする'; code-example-end]

これはコードを書かずに、ウィジェットカスタマイズページで、ドメイン全体または個別ページ単位でカスタマイズできます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='ウィジェットカスタマイズページの「新しい返信を防止」設定。ドメインまたはページのスレッドをロックします'; title='コメントスレッドを読み取り専用にする' app-screenshot-end]

## 更新！

2022年11月以降、スレッドは管理者やモデレーターが返信エリア上部の三点メニューから **リアルタイム** にロックまたはロック解除できます。

これにより新しいコメントは防止されますが、投票は引き続き可能で、必要に応じてユーザーは自分のコメントを削除できるようになります。一方、`readonly` ではこれらの操作は許可されません。

`Page` API の `isClosed` フィールドに対応しています。

---