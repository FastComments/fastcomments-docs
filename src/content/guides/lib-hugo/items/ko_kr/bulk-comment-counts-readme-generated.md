많은 페이지(예: 블로그 인덱스, 섹션 목록)에 댓글 수를 한 번에 표시하려면 bulk count widget을 사용하세요. 이 위젯은 페이지의 모든 카운트를 단일 요청으로 가져옵니다. 구성요소는 두 가지입니다: 각 항목 옆의 마커와 목록 뒤의 하나의 init 호출입니다.

In a list template (`layouts/_default/list.html`):

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

`count-marker.html`는 `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`을 렌더링하며, 해당 페이지에 대해 comments widget이 사용하는 것과 동일한 식별자(해당 페이지의 `urlId`, 또는 `urlId`가 설정되지 않은 경우 permalink)를 사용하므로 카운트가 실제 스레드와 일치합니다. `bulk-count.html`은 이 값들을 채우는 단일 요청을 보냅니다.

If you write the markers yourself (for example in a page's Markdown), use the shortcode to emit the init call instead:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```