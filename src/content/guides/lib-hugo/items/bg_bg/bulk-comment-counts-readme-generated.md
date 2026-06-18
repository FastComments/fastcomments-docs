За да покажете брой коментари до множество страници едновременно (като индекс на блог, списък на секция), използвайте уиджетa за масово преброяване. Той извлича всички броячи на страницата в една заявка. Има две части: маркер до всеки елемент и едно извикване за инициализация след списъка.

В шаблон за списък (`layouts/_default/list.html`):

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

`count-marker.html` визуализира `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, използвайки същия идентификатор, който уиджетът за коментари използва за тази страница (нейният `urlId`, или нейният permalink когато не е зададен `urlId`), така че броячите да съвпадат с реалните нишки. `bulk-count.html` изпраща единичната заявка, която ги попълва.

Ако напишете маркерите сами (например в Markdown на страницата), използвайте shortcode-а, за да изведете извикването за инициализация вместо това:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```