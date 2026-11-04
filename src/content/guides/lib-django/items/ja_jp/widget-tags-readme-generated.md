すべてのウィジェットにはそれぞれ専用のタグがあります。いずれも `**extra` キーワード引数を受け付けます。これは、以下の名前付き引数でカバーされていない項目について、ウィジェットの設定にそのままマージされます（キーは camelCase を使用してください）。

| タグ | ウィジェット |
|---|---|
| `{% fastcomments %}` | コメント |
| `{% fastcomments_live_chat %}` | ライブチャット |
| `{% fastcomments_comment_count %}` | コメント数バッジ |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | 一括コメント数 |
| `{% fastcomments_collab_chat target="#el" %}` | コラボレーション（インライン）チャット |
| `{% fastcomments_image_chat target="#el" %}` | 画像アノテーションチャット |
| `{% fastcomments_recent_comments %}` | 最近のコメント |
| `{% fastcomments_recent_discussions %}` | 最近のディスカッション |
| `{% fastcomments_reviews_summary %}` | レビュー概要 |
| `{% fastcomments_top_pages %}` | 最も議論されたページ |
| `{% fastcomments_user_activity user_id="..." %}` | ユーザーのアクティビティフィード |

名前付き引数は、ウィジェットの camelCase 設定キーに対応します:

| 引数 | 設定キー | タグ |
|---|---|---|
| `url_id` | `urlId` | コメント、ライブチャット、コメント数、コラボ/画像チャット、最近のコメント、レビュー概要 |
| `url` | `url` | コメント、ライブチャット、コラボ/画像チャット |
| `readonly` | `readonly` | コメント、ライブチャット、コラボ/画像チャット |
| `locale` | `locale` | コメント、ライブチャット、コラボ/画像チャット、ユーザーアクティビティ |
| `has_dark_background` | `hasDarkBackground` | すべて |
| `default_sort_direction` | `defaultSortDirection` | コメント、ライブチャット、コラボ/画像チャット |
| `number_only` | `numberOnly` | コメント数 |
| `is_live` | `isLive` | コメント数 |
| `count` | `count` | 最近のコメント、最近のディスカッション |
| `target` | (querySelector、送信されません) | コラボチャット、画像チャット |
| `chat_square_percentage` | `chatSquarePercentage` | 画像チャット |
| `user_id` | `userId` | ユーザーアクティビティ |

例:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# コラボチャットはページ上の既存の要素にアタッチされます #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# 一括コメント数: マーカーを配置すると、一括ローダーがすべてを埋め込みます #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```