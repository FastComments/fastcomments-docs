Щоб показати кількість коментарів поряд із багатьма сторінками одночасно (індекс блогу, список розділу), використайте віджет масового підрахунку. Він отримує всі лічильники на сторінці одним запитом. Є дві частини: маркер біля кожного елемента та один виклик ініціалізації після списку.

У шаблоні списку (`layouts/_default/list.html`):

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

`count-marker.html` рендерить `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, використовуючи той самий ідентифікатор, який віджет коментарів використовує для цієї сторінки (його `urlId`, або його `permalink` коли `urlId` не встановлено), так що лічильники відповідають реальним гілкам обговорення. `bulk-count.html` відправляє один запит, який заповнює їх.

Якщо ви вставляєте маркери самостійно (наприклад у Markdown сторінки), використайте шорткод, щоб замість цього згенерувати виклик ініціалізації:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```