データを移動する必要がある場合、FastComments はページや記事間でコメントを移動するためのセルフサービスツールを提供します。

コメントコピーのページフォームは次のようになります:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### 「From」欄の記入方法

コメントをどこから移動するか決めるには、ソースの `URL ID` を把握するだけで十分です。

コメントウィジェットの設定で `urlId` を渡していない場合、これはページ URL の「クリーン」バージョンになります。

コメントが持つ `URL ID` の値は、エクスポートによって確認できます。

### 「To」欄の記入方法

コメントをどこへ移動するか決めるには、ターゲットの `URL ID` と `URL` を把握する必要があります。

`URL ID` はコメントが格納されるバケットになります。`URL` フィールドは、メールやモデレーションツールから直接
コメントへ移動できるようにするために使用されます。

#### WordPress

WordPress を使用している場合、例えばマイグレーションツールの To/From `URL ID` フィールドには記事 ID を入力し、
URL の代わりに用います。

---