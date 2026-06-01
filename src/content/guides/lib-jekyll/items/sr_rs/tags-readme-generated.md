| Tag | Description |
| --- | --- |
| `fastcomments` | Живо коментисање са одговорима, гласањем, модерацијом и ажурирањима у реалном времену |
| `fastcomments_comment_count` | Број коментара за тренутну страницу |
| `fastcomments_comment_count_bulk` | Бројеви коментара за више страница на једној листи/индекс страници |
| `fastcomments_live_chat` | Видгет за стримовано ћаскање у реалном времену |
| `fastcomments_collab_chat` | Колаборативно inline коментарисање (текстуалне анотације) |
| `fastcomments_image_chat` | Коментари за анотацију слика |
| `fastcomments_recent_comments` | Недавни коментари на целом сајту |
| `fastcomments_recent_discussions` | Недавно активне дискусионе теме |
| `fastcomments_reviews_summary` | Сажетак рецензија са звездицама |
| `fastcomments_top_pages` | Странице о којима се највише дискутује |
| `fastcomments_user_activity_feed` | Фид активности по кориснику |

### Примери

```liquid
{% raw %}{# Број коментара. Видгет приказује своју ознаку, нпр. "0 коментара" #}
{% fastcomments_comment_count %}

{# Живо ћаскање #}
{% fastcomments_live_chat %}

{# Колаборативни чет. Усмерите га на елемент садржаја помоћу CSS селектора #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Чет за слике. Усмерите га на елемент слике помоћу CSS селектора #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Сажетак рецензија #}
{% fastcomments_reviews_summary %}

{# Фид активности корисника. Захтева user_id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Групни бројеви коментара за индекс блога #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```