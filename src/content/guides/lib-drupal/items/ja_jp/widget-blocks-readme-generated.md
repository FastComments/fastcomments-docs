いくつかのブロックは **構造 > ブロックレイアウト** (`/admin/structure/block`) から利用できます:

- **FastComments Widget** - メインのコメントウィジェット。現在のエンティティを自動検出します。すでに FastComments フィールドを持つエンティティはスキップします（重複を防ぐため）。
- **FastComments Live Chat** - リアルタイムのストリーミングチャット。同じページのコメントフィールドと並べて配置できます。
- **FastComments Collab Chat** - テキスト選択に基づく注釈とディスカッション。
- **FastComments Image Chat** - 画像上の座標ベースの注釈。
- **FastComments Recent Comments** - サイト全体の最新コメントを表示します。表示するコメント数は設定可能です。
- **FastComments Top Pages** - コメント数が多いページを表示します。

コンテンツ中心のブロック（Live Chat、Collab Chat、Image Chat）は現在のエンティティを自動検出し、エンティティでないページではパスベースの識別子にフォールバックします。