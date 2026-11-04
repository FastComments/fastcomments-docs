여러 페이지에 한 번에 댓글 수를 표시하려면(블로그 인덱스, 섹션 목록 등), 벌크 카운트 위젯을 사용하세요. 페이지의 모든 카운트를 단일 요청으로 가져옵니다. 두 가지 요소로 구성됩니다: 각 항목 옆의 마커와 목록 뒤에 오는 하나의 init 호출입니다.

목록 템플릿(`layouts/_default/list.html`)에서:

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

`count-marker.html`은 `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`을 렌더링하며, 해당 페이지의 댓글 위젯이 사용하는 것과 동일한 식별자(`urlId`, 또는 `urlId`가 설정되지 않은 경우 퍼머링크)를 사용하므로 카운트가 실제 스레드와 일치합니다. `bulk-count.html`은 이를 채우는 단일 요청을 생성합니다.

마커를 직접 작성하는 경우(예: 페이지의 Markdown에서), init 호출을 생성하기 위해 shortcode를 대신 사용하세요:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```