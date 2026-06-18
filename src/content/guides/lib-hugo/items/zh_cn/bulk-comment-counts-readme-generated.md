要在多个页面旁同时显示评论数（例如博客索引、章节列表），请使用批量计数组件。它会在单个请求中获取页面上的所有计数。它由两部分组成：每个项目旁的标记，以及列表之后的一次初始化调用。

在列表模板 (`layouts/_default/list.html`) 中：

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

`count-marker.html` 渲染 `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`，使用评论组件用于该页面的相同标识符（它的 `urlId`，或者当未设置 `urlId` 时使用其固定链接），因此计数与真实线程对应。`bulk-count.html` 发出填充这些计数的单个请求。

如果你自己编写这些标记（例如在页面的 Markdown 中），请改用短代码来发出初始化调用：

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```