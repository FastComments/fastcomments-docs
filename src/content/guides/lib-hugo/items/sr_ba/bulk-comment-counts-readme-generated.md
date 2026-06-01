Да бисте приказали број коментара поред више страница одједном (индекс блога, листа секције), користите видџет за масовно бројање. Он прикупља све бројеве на страници у једном захтеву. Састоји се из два дела: маркера поред сваког елемента и једног позива за иницијализацију након листе.

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

`count-marker.html` рендерује `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, користећи исти идентификатор који видџет за коментаре користи за ту страницу (њен `urlId`, или њен permalink када `urlId` није постављен), тако да се бројеви поклапају са стварним нитима. `bulk-count.html` шаље један захтев који их попуњава.

Ако сами пишете маркере (на пример у Markdown-у странице), користите шорткод да бисте уместо тога емитовали позив за иницијализацију:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```