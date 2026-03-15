사용자가 FastComments 필드가 활성화된 엔티티를 방문하면:

1. FastComments JavaScript 위젯이 CDN에서 로드됩니다.
2. SSO가 구성된 경우, 사용자의 Drupal 신원이 FastComments로 전달됩니다.
3. A `<noscript>` 대체(fallback)는 JavaScript가 없는 사용자에게 서버에서 렌더링된 댓글을 제공합니다 (Live Comments and Streaming Chat modes only).