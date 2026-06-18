Da biste prikazali broj komentara pored više stranica odjednom (index bloga, lista sekcije), koristite bulk count widget. On preuzima sve brojače komentara na stranici u jednom zahtevu. Postoje dve komponente: marker pored svake stavke, i jedan init poziv nakon liste.

U šablonu liste (`layouts/_default/list.html`):

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

Ako sami napišete markere (na primer u Markdownu stranice), koristite shortcode da umesto toga emitujete init poziv:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```