Per mostrare il conteggio dei commenti accanto a molte pagine contemporaneamente (un indice del blog, una lista di sezioni), usa il widget di conteggio bulk. Recupera ogni conteggio presente nella pagina in una singola richiesta. Ci sono due parti: un marcatore accanto a ogni elemento e una chiamata di init dopo la lista.

In un template di lista (`layouts/_default/list.html`):

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

`count-marker.html` rende `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, usando lo stesso identificatore che il widget dei commenti usa per quella pagina (il suo `urlId`, oppure il suo permalink quando non è impostato un `urlId`), così i conteggi corrispondono ai thread reali. `bulk-count.html` emette la singola richiesta che li popola.

Se scrivi i marker da solo (ad esempio nel Markdown di una pagina), usa lo shortcode per emettere invece la chiamata di inizializzazione:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```