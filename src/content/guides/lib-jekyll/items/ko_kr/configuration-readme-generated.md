설정은 세 곳에서 가져옵니다. 나중에 나오는 소스가 우선합니다:

1. `_config.yml`의 `fastcomments:` 키 아래에 있는 **전역 기본값**.
2. 페이지 범위 위젯에 대해 자동으로 도출되는 **페이지 컨텍스트**(아래 참조).
3. 태그 자체에 작성된 **태그 속성**.

따라서 태그의 `url_id`는 페이지에서 도출된 값을 재정의하고, 이 값은 전역 기본값을 재정의합니다.

### 속성 문법

속성은 `snake_case` 형식의 `key=value` 쌍입니다:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **따옴표로 묶인** 값(`"..."` 또는 `'...'`)은 리터럴 문자열입니다.
- **따옴표 없는** `true`/`false`는 불리언으로, 숫자는 숫자로 변환됩니다.
- **따옴표 없는** 그 외의 값은 페이지 컨텍스트에서 Liquid 변수로 해석됩니다. 예:
  `url_id=page.slug`. (Liquid는 태그의 속성 내부에서 `{% raw %}\{{ ... }}{% endraw %}`를 확장하지 않으므로,
  `"{% raw %}\{{ page.slug }}{% endraw %}"` 형태 대신 `page.slug` 형태를 사용해야 합니다.)

snake_case 속성 및 설정 키는 FastComments가 기대하는 camelCase 키로 자동 매핑됩니다
(`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground` 등). [위젯 설정](https://docs.fastcomments.com/guide-customizations-and-configuration.html)의
다른 모든 옵션도 동일한 방식으로 그대로 전달됩니다.

### 페이지에서 도출되는 값

페이지 범위 위젯(`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`)의 경우, 직접 설정하지 않는 한
현재 페이지에서 자동으로 채워집니다:

- `url_id` ← `page.url` (방문 도메인과 독립적인 안정적인 식별자)
- `url` ← `site.url` + `page.url` (`url`이 `_config.yml`에 설정된 경우에만)
- `page_title` ← `page.title`

사이트 전체 위젯(최근 댓글/토론, 인기 페이지, 리뷰 요약, 사용자 활동 피드,
대량 카운트)은 특정 페이지에 연결되지 않으므로 이 값들을 도출하지 않습니다.

### EU 데이터 레지던시

EU 고객은 전역적으로 `region: eu`를 추가하거나:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

태그별로 추가할 수 있습니다: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. 이후 위젯은 EU CDN에서 로드됩니다.