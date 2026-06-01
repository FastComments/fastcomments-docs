| Shortcode | Description |
| --- | --- |
| `fastcomments` | Threaded comments with replies, voting, and @mentions |
| `fastcomments-comment-count` | Comment count for a single page |
| `fastcomments-comment-count-bulk` | Comment counts for many pages in one request (see [Bulk comment counts](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Live chat widget |
| `fastcomments-collab-chat` | Collaborative inline commenting (requires `target`) |
| `fastcomments-image-chat` | Image annotation comments (requires `target`) |
| `fastcomments-recent-comments` | Recent comments across the site |
| `fastcomments-recent-discussions` | Recently active discussion threads |
| `fastcomments-reviews-summary` | Star-rating reviews summary |
| `fastcomments-top-pages` | Most-discussed pages |
| `fastcomments-user-activity-feed` | Per-user activity feed (requires `userId`) |

### Examples

Comment count inline with text:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Live chat:

```text
\{{< fastcomments-live-chat >}}
```

Collab chat, targeting a content element by CSS selector:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Image chat, targeting an image element by CSS selector:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Reviews summary:

```text
\{{< fastcomments-reviews-summary >}}
```

User activity feed:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```