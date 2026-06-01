Config는 세 곳에서 옵니다. 나중에 지정된 항목이 우선합니다:

1. **전역 기본값**은 `_config.yml`의 `fastcomments:` 키 아래에 있습니다.
2. **페이지 컨텍스트**는 페이지 범위 위젯에 대해 자동으로 파생됩니다(아래 참조).
3. **태그 속성**은 태그 자체에 작성된 값입니다.

따라서 태그에 있는 `url_id`는 페이지에서 파생된 값을 덮어쓰고, 그 값은 어떤 전역 기본값도 덮어씁니다.

### Attribute syntax

속성은 `snake_case`의 `key=value` 쌍입니다:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **따옴표로 묶인** 값(`"..."` 또는 `'...'`)은 리터럴 문자열입니다.
- **따옴표 없이** 쓴 `true`/`false`는 불리언이 되고, 숫자는 숫자가 됩니다.
- **따옴표 없이** 쓴 그 외의 항목은 페이지 컨텍스트의 Liquid 변수로 해석됩니다. 예: `url_id=page.slug`. (Liquid은 태그의 속성 안에서 `{% raw %}\{{ ... }}{% endraw %}`를 확장하지 않으므로, `"{% raw %}\{{ page.slug }}{% endraw %}"` 대신 bare `page.slug` 형태를 사용하세요.)

Snake_case 속성 및 설정 키는 FastComments가 기대하는 camelCase 키로 자동 매핑됩니다 (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, 등). [위젯 구성](https://docs.fastcomments.com/guide-customizations-and-configuration.html)의 다른 모든 옵션도 동일하게 직접 전달됩니다.

### Page-derived values

페이지 범위 위젯들(`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`)의 경우, 다음 값들이 직접 설정하지 않는 한 현재 페이지에서 자동으로 채워집니다:

- `url_id` ← `page.url` (방문 도메인과 무관한 안정적인 식별자)
- `url` ← `site.url` + `page.url` (`_config.yml`에 `url`이 설정된 경우에만)
- `page_title` ← `page.title`

사이트 전체 위젯(최근 댓글/토론, 상위 페이지, 리뷰 요약, 사용자 활동 피드, 일괄 카운트)은 특정 페이지에 묶여 있지 않으며 이러한 값을 유도하지 않습니다.

### EU data residency

EU 고객은 `region: eu`를 추가합니다. 전역적으로:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

또는 태그별로: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. 그러면 위젯은 EU CDN에서 로드됩니다.