WordPressからサイトを移行し、新しいサイトでFastCommentsを利用したい場合、WordPressプラグインは必要ありません。WordPressからコメントをエクスポートし、FastCommentsダッシュボードの[インポートページ](https://fastcomments.com/auth/my-account/manage-data/import)でファイルをアップロードしてください。

WordPressのエクスポート形式を2つサポートしています。

### WordPress XML(推奨)

これはWordPressの組み込みエクスポーターが出力するファイルなので、追加のプラグインは不要です。

1. WordPressの管理画面で `Tools -> Export` に移動します。
2. `All content` を選択し、`Download Export File` をクリックします。
3. FastCommentsの[インポートページ](https://fastcomments.com/auth/my-account/manage-data/import)で `WordPress (.xml)` を選択し、ファイルをアップロードします。

各コメントは、コメントが投稿されたページのURLに紐付けられており、このURLはすでにファイル内に含まれています。

インポートでは、投稿者名、メールアドレス、ウェブサイト、日付、内容、返信のスレッド構造、コメントの承認状態が保持されます。コメント投稿者のアバターはGravatarから取り込まれます。投票はこの形式には含まれません。

### WordPress CSV

これは[WebToffeeのWordPress Comments Import & Exportプラグイン](https://wordpress.org/plugins/comments-import-export-woocommerce/)が出力するファイルです。

1. WordPressの管理画面にプラグインをインストールし、コメントをCSVとしてエクスポートします。
2. 各 `comment_post_ID` の値を投稿のURLに置き換えます。
3. FastCommentsの[インポートページ](https://fastcomments.com/auth/my-account/manage-data/import)で `WordPress (.csv)` を選択し、ファイルをアップロードします。

各コメントは `comment_post_ID` 列に紐付けられています。WordPressはこの列に投稿IDを設定しますが、新しいサイトにはWordPressの投稿IDが存在しないため、手順2でURLに置き換えます。

インポートでは、投稿者名、メールアドレス、ウェブサイト、日付、内容、返信のスレッド構造、コメントの承認状態が保持されます。コメント投稿者のアバターはGravatarから取り込まれます。また、ファイルに含まれている場合は、WordPressのスパムフラグやwpDiscuzのいいね・よくないねも保持されます。

### 新しいページへのコメントの紐付け

新しいサイトがWordPressサイトと同じURLを維持している場合、追加の設定なしでコメントが対応するページに表示されます。

ドメインが変更される場合は、インポート後に[ドメイン移行ツール](/guide-migrations.html#migrating-domains)を実行してください。個別のページURLが変更される場合は、古いURLから新しいURLへ[各ページを移行](/guide-migrations.html#migrating-pages)できます。

コメントウィジェットの[urlId](/guide-customizations-and-configuration.html#url-id)フィールドに渡す値からドメインを削除するなど、ページの一括移行が必要な場合は、[サポートチケットを開いて](https://fastcomments.com/auth/my-account/help)ください。弊社にて対応いたします。

### 切り替え前の注意点

インポートは何度でも実行できます。同じファイルを再インポートしても[重複は作成されない](/guide-migrations.html#importing-data)ため、まず一度インポートして新しいサイトをテストし、切り替え直前に最新のコメントで再度インポートすることができます。

1GBを超えるエクスポートファイルについては、[サポートまでご連絡ください](https://fastcomments.com/auth/my-account/help)。

新しいサイトにFastCommentsを追加する方法については、[インストールガイド](/guide-installation.html)をご覧ください。