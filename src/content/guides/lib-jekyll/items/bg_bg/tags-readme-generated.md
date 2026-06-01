| Таг | Описание |
| --- | --- |
| `fastcomments` | Живи коментари с отговори, гласуване, модериране и актуализации в реално време |
| `fastcomments_comment_count` | Брой коментари за текущата страница |
| `fastcomments_comment_count_bulk` | Брой коментари за множество страници на една страница със списък/индекс |
| `fastcomments_live_chat` | Уиджет за потоков чат в реално време |
| `fastcomments_collab_chat` | Съвместно вградено коментиране (текстови анотации) |
| `fastcomments_image_chat` | Коментари за анотиране на изображения |
| `fastcomments_recent_comments` | Скорошни коментари в целия сайт |
| `fastcomments_recent_discussions` | Наскоро активни дискусии |
| `fastcomments_reviews_summary` | Обобщение на ревюта с оценка в звезди |
| `fastcomments_top_pages` | Най-обсъжданите страници |
| `fastcomments_user_activity_feed` | Лента с активност за всеки потребител |

### Примери

```liquid
{% raw %}{# Брой коментари. Уиджетът изобразява собствен етикет, напр. "0 коментара" #}
{% fastcomments_comment_count %}

{# Чат на живо #}
{% fastcomments_live_chat %}

{# Съвместен чат. Насочете го към елемент на съдържанието чрез CSS селектор #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Чат за изображения. Насочете го към елемент изображение чрез CSS селектор #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Обобщение на ревюта #}
{% fastcomments_reviews_summary %}

{# Лента с активността на потребителя. Изисква идентификатор на потребителя #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Масово преброяване на коментари за индекс на блог #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```