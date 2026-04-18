---
| Shortcode | 説明 |
| --- | --- |
| `fastcomments` | 返信、投票などを備えたコメントウィジェット |
| `fastcommentsCommentCount` | ページのコメント数を表示 |
| `fastcommentsImageChat` | 画像への注釈コメント |
| `fastcommentsLiveChat` | ライブチャットウィジェット |
| `fastcommentsCollabChat` | 共同インラインコメント |
| `fastcommentsRecentComments` | サイト全体の最近のコメント |
| `fastcommentsRecentDiscussions` | 最近アクティブなディスカッションスレッド |
| `fastcommentsReviewsSummary` | 星評価レビューの概要 |
| `fastcommentsTopPages` | 最も議論されたページ |
| `fastcommentsUserActivityFeed` | ユーザーアクティビティフィード |

### 例

```njk
{# テキスト内でのコメント数をインライン表示 #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# ライブチャット #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# コラボチャット — CSSセレクターでコンテンツ要素をターゲットにする #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# 画像チャット — CSSセレクターで画像要素をターゲットにする #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# レビュー概要 #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# ユーザーアクティビティフィード #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```
---