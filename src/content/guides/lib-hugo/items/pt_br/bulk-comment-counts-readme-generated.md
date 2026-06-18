Para mostrar a contagem de comentários ao lado de várias páginas de uma vez (um índice de blog, uma lista de seção), use o widget de contagem em lote. Ele busca todas as contagens na página em uma única requisição. Existem duas partes: um marcador ao lado de cada item, e uma chamada de inicialização após a lista.

Em um template de lista (`layouts/_default/list.html`):

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

`count-marker.html` renderiza `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, usando o mesmo identificador que o widget de comentários usa para essa página (seu `urlId`, ou seu permalink quando nenhum `urlId` estiver definido), assim as contagens correspondem aos tópicos reais. `bulk-count.html` emite a única requisição que os preenche.

Se você escrever os marcadores manualmente (por exemplo no Markdown de uma página), use o shortcode para emitir a chamada de inicialização em vez disso:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```