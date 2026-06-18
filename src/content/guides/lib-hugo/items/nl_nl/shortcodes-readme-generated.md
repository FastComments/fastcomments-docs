| Shortcode | Beschrijving |
| --- | --- |
| `fastcomments` | Gelaagde reacties met antwoorden, stemmen en @mentions |
| `fastcomments-comment-count` | Aantal reacties voor een enkele pagina |
| `fastcomments-comment-count-bulk` | Reactieaantallen voor meerdere pagina's in één verzoek (zie [Bulk reactietellingen](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Livechat-widget |
| `fastcomments-collab-chat` | Samenwerkende inline opmerkingen (vereist `target`) |
| `fastcomments-image-chat` | Annotaties op afbeeldingen (vereist `target`) |
| `fastcomments-recent-comments` | Recente reacties op de hele site |
| `fastcomments-recent-discussions` | Recent actieve discussiedraden |
| `fastcomments-reviews-summary` | Samenvatting van beoordelingen met sterren |
| `fastcomments-top-pages` | Meest besproken pagina's |
| `fastcomments-user-activity-feed` | Activiteitenfeed per gebruiker (vereist `userId`) |

### Voorbeelden

Aantal reacties inline in de tekst:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Livechat:

```text
\{{< fastcomments-live-chat >}}
```

Collab chat, een contentelement targeten met een CSS-selector:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Image chat, een afbeeldings-element targeten met een CSS-selector:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Samenvatting van beoordelingen:

```text
\{{< fastcomments-reviews-summary >}}
```

Gebruikersactiviteitenfeed:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```