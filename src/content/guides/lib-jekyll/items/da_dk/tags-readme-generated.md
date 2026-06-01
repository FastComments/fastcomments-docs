| Tag | Description |
| --- | --- |
| `fastcomments` | Live-kommentering med svar, afstemninger, moderation og realtidsopdateringer |
| `fastcomments_comment_count` | Antal kommentarer for den aktuelle side |
| `fastcomments_comment_count_bulk` | Antal kommentarer for flere sider på en liste-/indeksside |
| `fastcomments_live_chat` | Realtids-streaming chat-widget |
| `fastcomments_collab_chat` | Samarbejdsbaseret inline-kommentering (tekstannoteringer) |
| `fastcomments_image_chat` | Kommentarer til billedannoteringer |
| `fastcomments_recent_comments` | Seneste kommentarer på tværs af sitet |
| `fastcomments_recent_discussions` | Nyligt aktive diskussionstråde |
| `fastcomments_reviews_summary` | Oversigt over stjernebedømmelser |
| `fastcomments_top_pages` | Mest diskuterede sider |
| `fastcomments_user_activity_feed` | Aktivitetsfeed pr. bruger |

### Eksempler

```liquid
{% raw %}{# Antal kommentarer. Widget'en gengiver sin egen etiket, f.eks. "0 comments" #}
{% fastcomments_comment_count %}

{# Live-chat #}
{% fastcomments_live_chat %}

{# Collab-chat. Ret den mod et indholdselement med en CSS-selektor #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Billed-chat. Ret den mod et billedelement med en CSS-selektor #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Anmeldelsesoversigt #}
{% fastcomments_reviews_summary %}

{# Bruger-aktivitetfeed. Kræver et bruger-id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Samlede kommentartællinger for et blogindeks #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```