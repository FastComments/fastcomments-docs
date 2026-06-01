| Tag | Description |
| --- | --- |
| `fastcomments` | ライブコメント（返信、投票、モデレーション、およびリアルタイム更新） |
| `fastcomments_comment_count` | 現在のページのコメント数 |
| `fastcomments_comment_count_bulk` | 一覧（インデックス）ページ上の複数ページのコメント数 |
| `fastcomments_live_chat` | リアルタイムストリーミングチャットウィジェット |
| `fastcomments_collab_chat` | 共同のインラインコメント（テキスト注釈） |
| `fastcomments_image_chat` | 画像への注釈コメント |
| `fastcomments_recent_comments` | サイト全体の最近のコメント |
| `fastcomments_recent_discussions` | 最近アクティブなディスカッションスレッド |
| `fastcomments_reviews_summary` | 星評価レビューの概要 |
| `fastcomments_top_pages` | 最も議論されているページ |
| `fastcomments_user_activity_feed` | ユーザーごとのアクティビティフィード |

### 例

```liquid
{% raw %}{# Comment count. The widget renders its own label, e.g. "0 comments" #}
{% fastcomments_comment_count %}

{# Live chat #}
{% fastcomments_live_chat %}

{# Collab chat. Point it at a content element with a CSS selector #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Image chat. Point it at an image element with a CSS selector #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Reviews summary #}
{% fastcomments_reviews_summary %}

{# User activity feed. Requires a user id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Bulk comment counts for a blog index #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```