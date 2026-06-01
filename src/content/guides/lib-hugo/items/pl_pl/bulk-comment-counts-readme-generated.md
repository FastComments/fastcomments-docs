Aby wyświetlić liczbę komentarzy obok wielu stron naraz (indeks bloga, lista sekcji), użyj widżetu zbiorczego licznika. Pobiera on wszystkie liczniki na stronie w jednym żądaniu. Składa się z dwóch elementów: znacznika obok każdego elementu oraz jednego wywołania inicjalizującego po liście.

W szablonie listy (`layouts/_default/list.html`):

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

`count-marker.html` renderuje `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, używając tego samego identyfikatora, którego widżet komentarzy używa dla tej strony (jej `urlId`, albo jej permalink jeśli `urlId` nie jest ustawione), dzięki czemu liczniki odpowiadają rzeczywistym wątkom. `bulk-count.html` wysyła pojedyncze żądanie, które je wypełnia.

Jeśli utworzysz znaczniki samodzielnie (na przykład w Markdown strony), użyj shortcode'a, aby zamiast tego wygenerować wywołanie inicjalizujące:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```