モジュールには、`Structure > Block layout` (`/admin/structure/block`) から配置できるいくつかのブロックが同梱されています。

- **FastComments Widget** - メインのコメントウィジェット。現在のエンティティを自動検出します。すでに FastComments フィールドが追加されているエンティティはスキップするため、同じページに重複したウィジェットが表示されることはありません。
- **FastComments Live Chat** - リアルタイムストリーミングチャット。コメントフィールドと同じページに並べて配置できます。
- **FastComments Collab Chat** - テキスト選択による注釈とディスカッション。
- **FastComments Image Chat** - 画像上の座標ベースの注釈。訪問者は画像をクリックして特定の位置に紐づいたコメントを残します。
- **FastComments Recent Comments** - サイト全体の最近のコメントを表示します。表示件数はブロックで設定可能です。
- **FastComments Top Pages** - コメント数が多いページを表示します。

コンテンツ中心のブロック（Live Chat、Collab Chat、Image Chat）は現在のエンティティを自動検出し、エンティティでないページではパスベースの識別子にフォールバックします。つまり、追加の設定なしにタクソノミーページ、ビュー、カスタムルートで動作します。