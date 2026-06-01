要在多個頁面旁同時顯示留言數（例如部落格索引、區段列表），請使用批次計數小工具。它會在單一請求中擷取頁面上所有的計數。這由兩個部分組成：每個項目旁的標記，以及列表之後的一個初始化呼叫。

在列表範本（`layouts/_default/list.html`）中：

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

`count-marker.html` 會呈現 `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`，使用評論小工具對該頁面使用的相同識別碼（其 `urlId`，或在未設定 `urlId` 時使用其永久連結），這樣計數就會與實際討論串對齊。`bulk-count.html` 發出填入這些計數的單一請求。

如果你自行撰寫這些標記（例如在頁面的 Markdown 中），請改用這個短碼來輸出初始化呼叫：

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```