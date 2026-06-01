| Tag | Description |
| --- | --- |
| `fastcomments` | Live commenting with replies, voting, moderation, and realtime updates |
| `fastcomments_comment_count` | Comment count for the current page |
| `fastcomments_comment_count_bulk` | Comment counts for many pages on one list/index page |
| `fastcomments_live_chat` | Realtime streaming chat widget |
| `fastcomments_collab_chat` | Collaborative inline commenting (text annotations) |
| `fastcomments_image_chat` | Image annotation comments |
| `fastcomments_recent_comments` | Recent comments across the site |
| `fastcomments_recent_discussions` | Recently active discussion threads |
| `fastcomments_reviews_summary` | Star-rating reviews summary |
| `fastcomments_top_pages` | Most-discussed pages |
| `fastcomments_user_activity_feed` | Per-user activity feed |

### Examples

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