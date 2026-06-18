Чтобы показать количество комментариев рядом с несколькими страницами одновременно (например, индекс блога или список раздела), используйте виджет массового подсчёта (bulk count). Он получает все значения на странице одним запросом. Состоит из двух частей: маркер рядом с каждым элементом и один вызов инициализации после списка.

В шаблоне списка (`layouts/_default/list.html`):

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

`count-marker.html` рендерит `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, используя тот же идентификатор, который виджет комментариев использует для этой страницы (ее `urlId`, или её permalink, когда `urlId` не задан), так что счётчики совпадают с реальными тредами. `bulk-count.html` отправляет единый запрос, который их заполняет.

Если вы создаёте маркеры самостоятельно (например, в Markdown страницы), используйте шорткод, чтобы вместо этого выполнить вызов инициализации:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```