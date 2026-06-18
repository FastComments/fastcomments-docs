Če želite ob številnih straneh hkrati prikazati število komentarjev (npr. indeks bloga ali seznam razdelkov), uporabite pripomoček za množično štetje (bulk count widget). V enem samem zahtevku pridobi vse števce na strani. Sestavljen je iz dveh delov: marker ob vsakem elementu in en inicializacijski klic po seznamu.

V predlogi seznama (`layouts/_default/list.html`):

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

`count-marker.html` izriše `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, pri čemer uporablja isti identifikator, ki ga comments widget uporablja za to stran (its `urlId`, or its permalink when no `urlId` is set), tako se števci ujemajo z dejanskimi nitmi. `bulk-count.html` odda en sam zahtevek, ki jih napolni.

Če markerje napišete sami (na primer v Markdownu strani), uporabite shortcode, da namesto tega oddate inicializacijski klic:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```