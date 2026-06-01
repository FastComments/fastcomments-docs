| Tag | Beschrijving |
| --- | --- |
| `fastcomments` | Live reacties met antwoorden, stemmen, moderatie en realtime-updates |
| `fastcomments_comment_count` | Aantal reacties voor de huidige pagina |
| `fastcomments_comment_count_bulk` | Reactieaantallen voor meerdere pagina's op een lijst-/indexpagina |
| `fastcomments_live_chat` | Realtime streaming chatwidget |
| `fastcomments_collab_chat` | Samenwerkende inline-opmerkingen (tekstannotaties) |
| `fastcomments_image_chat` | Reacties op afbeeldingsannotaties |
| `fastcomments_recent_comments` | Recente reacties op de site |
| `fastcomments_recent_discussions` | Onlangs actieve discussiedraden |
| `fastcomments_reviews_summary` | Samenvatting van beoordelingen met sterren |
| `fastcomments_top_pages` | Meest besproken pagina's |
| `fastcomments_user_activity_feed` | Activiteitenfeed per gebruiker |

### Voorbeelden

```liquid
{% raw %}{# Aantal reacties. De widget rendert zijn eigen label, bijv. "0 reacties" #}
{% fastcomments_comment_count %}

{# Livechat #}
{% fastcomments_live_chat %}

{# Collab-chat. Wijs het toe aan een contentelement met een CSS-selector #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Image-chat. Wijs het toe aan een afbeeldingelement met een CSS-selector #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Samenvatting van beoordelingen #}
{% fastcomments_reviews_summary %}

{# Gebruikersactiviteitenfeed. Vereist een gebruikers-id #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Bulk-reactieaantallen voor een blogindex #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```