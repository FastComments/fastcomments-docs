Para mostrar un recuento de comentarios junto a muchas páginas a la vez (un índice de blog, una lista de sección), usa el widget de recuento masivo. Obtiene todos los recuentos de la página en una sola petición. Hay dos piezas: un marcador junto a cada elemento y una llamada de init después de la lista.

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

`count-marker.html` renderiza `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, usando el mismo identificador que el widget de comentarios usa para esa página (su `urlId`, o su permalink cuando no hay `urlId` establecido), de modo que los recuentos coincidan con los hilos reales. `bulk-count.html` emite la única petición que los rellena.

Si escribes los marcadores tú mismo (por ejemplo, en el Markdown de una página), usa el shortcode para emitir la llamada de init en su lugar:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```