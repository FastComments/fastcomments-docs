모든 FastComments 위젯 옵션은 `hugo.toml`의 `[params.fastcomments]` 아래에 설정되며, 페이지별로는 프런트매터의 `[fastcomments]`에서 재정의할 수 있습니다. 우선순위(낮음→높음): 사이트 params, 페이지 프런트매터, 숏코드 매개변수.

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

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### EU 데이터 레지던시

EU 고객은 `region = "eu"`를 설정하여 위젯을 `cdn-eu.fastcomments.com`으로 라우팅합니다:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### 키 케이싱에 대한 참고

Hugo는 `hugo.toml`과 프런트매터의 모든 키를 소문자로 변환하지만, FastComments 위젯은 camelCase 키(`tenantId`, `hasDarkBackground`)를 요구합니다. 이 컴포넌트는 알려진 최상위 옵션에 대해서는 자동으로 올바른 대/소문자를 복원하므로, 옵션을 일반적인 camelCase 형태로 작성하세요. 객체 값 내부에 중첩된 키(예: `translations` 맵의 키 또는 `pageReactConfig`의 필드)는 복원되지 않습니다. 이러한 항목들은 대신 FastComments 대시보드의 커스터마이제이션 UI에서 구성하세요.