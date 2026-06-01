| Ознака | Опис |
| --- | --- |
| `fastcomments` | Уживо коментарисање са одговорима, гласањем, модерацијом и ажурирањима у реалном времену |
| `fastcomments_comment_count` | Број коментара за тренутну страницу |
| `fastcomments_comment_count_bulk` | Бројеви коментара за више страница на једној страници индекса |
| `fastcomments_live_chat` | Видџет за чет у реалном времену |
| `fastcomments_collab_chat` | Колаборативно инлајн коментарисање (текстуалне анотације) |
| `fastcomments_image_chat` | Коментари са анотацијама на сликама |
| `fastcomments_recent_comments` | Најновији коментари широм сајта |
| `fastcomments_recent_discussions` | Недавно активне дискусионе теме |
| `fastcomments_reviews_summary` | Сажетак рецензија са оцјенама у звездицама |
| `fastcomments_top_pages` | Највише дискутоване странице |
| `fastcomments_user_activity_feed` | Фид активности по кориснику |

### Примери

```liquid
{% raw %}{# Број коментара. Видџет приказује своју ознаку, нпр. "0 коментара" #}
{% fastcomments_comment_count %}

{# Чет уживо #}
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

{# Фид активности корисника. Захтева user id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Збирни бројеви коментара за индекс блога #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```