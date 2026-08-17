コメントスレッドをモデレートおよび閲覧する際、モデレート中にコンテキストを得るためにスレッドへ直接ジャンプできることが望まれます。

これは、ユーザーのフローがコメントモデレーションページで開始し、個々のコメントから
そのコメントが含まれるページへ移動し、ページの読み込みが完了し、コメントの読み込みが完了した後にそのコメントまでスクロールしなければならないことを意味します。

しかし、FastComments はより速い方法を提供します。コメントモデレートページでは、各コメントの右下に「View Comment」ボタンがあります。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; alt='モデレーションリストの単一コメントで、右下に「View Comment」ボタンがあります'; title='コメント' app-screenshot-end]

このコメントに返信がある場合、ボタンのテキストは返信数が表示されますが、クリックすると同じ動作が行われます。

このボタンをクリックすると **Comment Thread Viewer** に移動します。

Comment Thread Viewer は、FastComments がホストする小さくて高速に読み込まれるアプリケーションで、コメントが属するページのコメントスレッドを表示し、そのコメントまでスクロールします。

これにより、モデレーターは別のページの読み込みを待つことなく、必要なコンテキストを迅速に取得できます。