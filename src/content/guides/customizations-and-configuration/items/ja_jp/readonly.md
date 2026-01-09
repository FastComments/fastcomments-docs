[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

readonly フラグを true に設定すると、新しいコメントや投票を受け付けないようにコメント投稿をロックできます。

コメントは編集や削除もできなくなります。

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

これはコード無しで、ウィジェットのカスタマイズページからドメイン全体またはページ単位で設定できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## 更新！

2022年11月現在、スレッドは管理者およびモデレーターが返信欄上部の三点メニューから**リアルタイムで**ロックまたはアンロックできます。

これにより新しいコメントは防止されますが、投票は引き続き可能で、ユーザーが自分のコメントを削除することもできます。一方、`readonly` ではこれらは許可されません。 

これは `Page` API の `isClosed` フィールドに対応します。