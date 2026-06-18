Da biste prikazali broj komentara pokraj više stranica odjednom (popis bloga, popis odjeljka), upotrijebite widget za masovno brojanje. On dohvaća sve brojeve na stranici u jednom zahtjevu. Postoje dva dijela: marker pored svakog stavka i jedan poziv za inicijalizaciju nakon popisa.

U predlošku popisa (`layouts/_default/list.html`):

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

`count-marker.html` renderira `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, koristeći isti identifikator koji widget za komentare koristi za tu stranicu (njegov `urlId`, ili njegov permalink ako `urlId` nije postavljen), tako da se brojevi podudaraju s pravim nitima. `bulk-count.html` šalje jedan zahtjev koji ih popunjava.

Ako markere napišete sami (na primjer u Markdownu stranice), umjesto toga upotrijebite shortcode da pokrenete poziv za inicijalizaciju:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```