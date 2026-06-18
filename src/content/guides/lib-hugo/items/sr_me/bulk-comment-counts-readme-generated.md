Да бисте приказали број коментара поред више страница одједном (индекс блога, листа секција), користите видгет за масовно бројање. Он прикупља све бројеве коментара на страници у једном захтеву. Постоје два дела: маркер поред сваке ставке, и један иницијализациони позив након листе.

У шаблону листе (`layouts/_default/list.html`):

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

`count-marker.html` рендерује `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, користећи исти идентификатор који видгет за коментаре користи за ту страницу (њен `urlId`, или њен permalink када `urlId` није подешен), тако да се бројеви поклапају са правим нитима. `bulk-count.html` шаље један захтев који их попуњава.

Ако сами уписујете маркере (на пример у Markdown-у странице), уместо тога користите шорткод да емитујете init позив:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```