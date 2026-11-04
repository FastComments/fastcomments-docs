Da biste prikazali broj komentara pored više stranica odjednom (indeks bloga, lista sekcije), koristite widget za masovno brojanje. On dohvata sve brojeve na stranici jednim zahtevom. Sastoji se od dva dela: markera pored svake stavke i jednog init poziva nakon liste.

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

`count-marker.html` renderuje `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, koristeći isti identifikator koji widget za komentare koristi za tu stranicu (njen `urlId`, ili njen permalink kada `urlId` nije postavljen), tako da se brojevi poklapaju sa stvarnim nitima. `bulk-count.html` emituje jedan zahtev koji ih popunjava.

Ako sami pišete markere (na primer u Markdown-u stranice), koristite shortcode da emitujete init poziv:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```