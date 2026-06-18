Чтобы показать число комментариев рядом с множеством страниц одновременно (например, индекс блога или список раздела), используйте виджет bulk count. Он запрашивает все счётчики на странице одним запросом. Он состоит из двух частей: маркера рядом с каждым элементом и одного вызова инициализации после списка.

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

`count-marker.html` генерирует `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, используя тот же идентификатор, который виджет комментариев использует для этой страницы (его `urlId`, or its permalink when no `urlId` is set), так что счётчики соответствуют реальным обсуждениям. `bulk-count.html` отправляет один запрос, который их заполняет.

Если вы создаёте маркеры вручную (например, в Markdown страницы), используйте шорткод, чтобы вместо этого вывести вызов инициализации:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```