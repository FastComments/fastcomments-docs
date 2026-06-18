Para mostrar un recuento de comentarios junto a muchas páginas a la vez (un índice de blog, una lista de secciones), use el widget de recuento masivo. Recupera todos los recuentos de la página en una sola solicitud. Hay dos piezas: un marcador junto a cada elemento y una llamada de inicialización después de la lista.

En una plantilla de lista (`layouts/_default/list.html`):

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

Si escribes los marcadores tú mismo (por ejemplo en el Markdown de una página), usa el shortcode para emitir la llamada de inicialización en su lugar:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```