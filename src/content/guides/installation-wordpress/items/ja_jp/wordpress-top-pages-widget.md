Top Pages ウィジェットは、サイトで最もコメントの多いページを表示します。新しい訪問者に最もエンゲージメントの高いコンテンツを示し、サイト滞在時間を増やすのに役立ちます。

## オプション

- **タイトル**（任意）：リストの上に表示される見出し。デフォルトは "Top Pages" です。

Top Pages ウィジェットは利用可能なデータに基づいて独自のレイアウトを選択し、count 属性を受け付けません。

## 追加方法

### 投稿またはページ内

ブロックエディタで、**Shortcode** ブロックを追加し、次を貼り付けます：

[inline-code-attrs-start title = 'Top Pages ショートコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### サイドバーまたはフッター（クラシックテーマ）

WordPress 管理画面の **外観 > ウィジェット** に移動します。ブロックインサーターから "FastComments" を検索し、**FastComments: Top Pages** を選択します。サイドバー、ヘッダー、またはフッターのエリアにドラッグし、ウィジェットパネルでタイトルを設定します。

### ブロックテーマ（フルサイト編集）

**外観 > エディター** 内のサイトエディターを開きます。ウィジェットを表示させたいテンプレートパーツに移動し、**Legacy Widget** ブロックを挿入して、ドロップダウンから **FastComments: Top Pages** を選択します。

## トラブルシューティング

ウィジェットは、FastComments のセットアップが完了して tenant ID が保存されてからのみ表示されます。ウィジェット領域が空白の場合は、WordPress 管理画面の FastComments でセットアップを完了し、ページを再読み込みしてください。