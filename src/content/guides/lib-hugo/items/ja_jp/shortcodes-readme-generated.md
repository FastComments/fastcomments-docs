| Shortcode | Description |
| --- | --- |
| `fastcomments` | 返信、投票、および@メンションをサポートするスレッド形式コメント |
| `fastcomments-comment-count` | 単一ページのコメント数 |
| `fastcomments-comment-count-bulk` | 1リクエストで複数ページのコメント数を取得（[一括コメント数](#bulk-comment-counts-readme-generated)を参照） |
| `fastcomments-live-chat` | ライブチャットウィジェット |
| `fastcomments-collab-chat` | 共同インラインコメント（`target`が必要） |
| `fastcomments-image-chat` | 画像注釈コメント（`target`が必要） |
| `fastcomments-recent-comments` | サイト全体の最近のコメント |
| `fastcomments-recent-discussions` | 最近アクティブなディスカッションスレッド |
| `fastcomments-reviews-summary` | 星評価のレビュー概要 |
| `fastcomments-top-pages` | 最も議論されたページ |
| `fastcomments-user-activity-feed` | ユーザー単位のアクティビティフィード（`userId`が必要） |

### 例

テキスト内でのコメント数：

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

ライブチャット：

```text
\{{< fastcomments-live-chat >}}
```

共同チャット — CSSセレクタでコンテンツ要素をターゲット：

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

画像チャット — CSSセレクタで画像要素をターゲット：

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

レビュー概要：

```text
\{{< fastcomments-reviews-summary >}}
```

ユーザーアクティビティフィード：

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```