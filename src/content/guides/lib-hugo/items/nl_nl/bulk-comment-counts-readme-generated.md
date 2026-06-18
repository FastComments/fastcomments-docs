Om een reactieaantal tegelijk naast meerdere pagina's te tonen (een blogindex, een sectielijst), gebruik je de bulk count widget. Deze haalt elk aantal op de pagina op in één verzoek. Er zijn twee onderdelen: een marker naast elk item, en één init-aanroep na de lijst.

In een lijsttemplate (`layouts/_default/list.html`):

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

`count-marker.html` rendert `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, met dezelfde identifier die de comments widget voor die pagina gebruikt (de `urlId`, of de permalink wanneer er geen `urlId` is ingesteld), zodat de tellingen overeenkomen met de echte threads. `bulk-count.html` zendt het enkele verzoek uit dat ze invult.

Als je de markers zelf schrijft (bijvoorbeeld in de Markdown van een pagina), gebruik dan de shortcode om in plaats daarvan de init-aanroep te genereren:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```