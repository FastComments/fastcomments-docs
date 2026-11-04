Recent Commentsウィジェットは、サイト全体に投稿された最新のコメントを表示します。サイドバーやフッターなど、新着アクティビティを見せてさらなる閲覧を促したい場所で便利です。

## オプション

- **タイトル**(任意): リストの上に表示される見出し。デフォルトは「Recent Comments」です。
- **件数**(任意): 表示するコメントの数。1から50の範囲で指定します。デフォルトは5です。

## 追加方法

### 投稿またはページ内

ブロックエディターで**ショートコード**ブロックを追加し、以下を貼り付けます:

[inline-code-attrs-start title = 'Recent Commentsショートコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

`count`属性には1から50までの任意の値を指定できます。

### サイドバーまたはフッター(クラシックテーマ)

WordPress管理画面で**外観 > ウィジェット**に移動します。ブロック挿入ツールで「FastComments」を検索し、**FastComments: Recent Comments**を選択します。サイドバー、ヘッダー、またはフッターエリアにドラッグし、ウィジェットパネルからタイトルと件数を設定します。

### ブロックテーマ(サイトエディター)

**外観 > エディター**から**サイトエディター**を開きます。ウィジェットを表示したいテンプレートパートに移動し、**Legacy Widget**ブロックを挿入して、ドロップダウンから**FastComments: Recent Comments**を選択します。

## トラブルシューティング

このウィジェットは、FastCommentsのセットアップが完了し、テナントIDが保存された後にのみ表示されます。ウィジェットエリアが空白の場合は、WordPress管理画面の**FastComments**でセットアップを完了し、ページを再読み込みしてください。