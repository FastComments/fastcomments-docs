---
Hugo에 내장된 Disqus 지원이 하는 방식처럼 모든 게시물에 댓글을 추가하려면, 테마의 single 템플릿 (`layouts/_default/single.html`)에 한 줄을 추가하세요:

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

partial은 `tenantId`가 구성된 경우에만 렌더링됩니다. 개별 페이지에서 프론트 매터로 댓글을 비활성화하려면:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---