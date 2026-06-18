To show a comment count next to many pages at once (a blog index, a section list), use the bulk count widget. It fetches every count on the page in a single request. There are two pieces: a marker next to each item, and one init call after the list.

In a list template (`layouts/_default/list.html`):

```go-html-template
<ul>
  \{{ range .Pages }}
    <li>
      <a href="\{{ .RelPermalink }}">\{{ .Title }}</a>
      \{{ partial "fastcomments/count-marker.html" . }}
    </li>
  \{{ end }}
</ul>
\{{ partial "fastcomments/bulk-count.html" (dict "page" .) }}
```

`count-marker.html` renders `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, using the same identifier the comments widget uses for that page (its `urlId`, or its permalink when no `urlId` is set), so the counts line up with the real threads. `bulk-count.html` emits the single request that fills them in.

If you write the markers yourself (for example in a page's Markdown), use the shortcode to emit the init call instead:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```