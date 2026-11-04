모든 FastComments 위젯 옵션은 `hugo.toml`의 `[params.fastcomments]` 아래에 설정되며, 각 페이지의 front matter에서 `[fastcomments]` 아래에 설정하여 개별적으로 재정의할 수 있습니다. 우선순위는 낮은 것부터 높은 것 순으로: 사이트 파라미터, 페이지 front matter, 숏코드 파라미터입니다.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

`url`과 `urlId` 모두 제공되지 않으면, `url`은 페이지의 퍼머링크를 기본값으로 사용하여 댓글 스레드가 안정적인 URL에 연결된 상태로 유지됩니다.

### EU 데이터 레지던시

EU 고객은 `region = "eu"`를 설정하여 위젯을 `cdn-eu.fastcomments.com`으로 라우팅합니다:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### 키 대소문자 관련 참고 사항

Hugo는 `hugo.toml`과 front matter의 모든 키를 소문자로 변환하지만, FastComments 위젯은 camelCase 키(`tenantId`, `hasDarkBackground`)를 필요로 합니다. 이 컴포넌트는 알려진 모든 최상위 옵션의 올바른 대소문자를 자동으로 복원하므로, 옵션을 일반적인 camelCase 형식으로 작성하면 됩니다. 객체 값 내부에 중첩된 키(예: `translations` 맵의 키 또는 `pageReactConfig`의 필드)는 복원되지 않습니다. 이러한 항목은 대신 FastComments 대시보드의 커스터마이제이션 UI를 통해 설정하세요.