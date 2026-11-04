새 웹훅 및 웹훅 편집 페이지에는 폼에 현재 입력된 URL로 저장 여부와 관계없이 요청을 보내는 `Send Test Payload` 버튼이 있습니다. Create 및 Update 이벤트는 더미 WebhookComment 객체를 전송하며, Delete 테스트 시에는 ID만 포함된 더미 요청 본문을 전송합니다.

## 페이로드 검증

웹훅 통합을 테스트할 때, 수신되는 요청에 다음 헤더가 포함되어 있는지 확인하세요:

1. **`X-FastComments-Timestamp`** - Unix 타임스탬프 (초)
2. **`X-FastComments-Signature`** - HMAC-SHA256 서명

서명 스키마가 도입되기 전에 생성된 웹훅은 API Secret을 담고 있는 **`token`** 헤더도 함께 수신합니다. 새 웹훅에는 포함되지 않습니다.

페이로드의 진위성을 확인하려면 HMAC 서명 검증을 사용하세요.

## 테스트 도구

개발 중 수신되는 웹훅 페이로드를 검사하기 위해 [webhook.site](https://webhook.site) 또는 [ngrok](https://ngrok.com)과 같은 도구를 사용할 수 있습니다.

## 이벤트 유형

- **Create 이벤트**: 새 댓글이 생성될 때 트리거됩니다.
- **Update 이벤트**: 댓글이 수정될 때 트리거됩니다.
- **Delete 이벤트**: 댓글이 삭제될 때 트리거됩니다.

각 웹훅은 하나의 이벤트와 하나의 HTTP 메서드(POST, PUT 또는 DELETE)에 연결됩니다. 각 이벤트는 요청 본문에 전체 댓글 데이터를 포함합니다(페이로드 형식은 [데이터 구조](/guide-webhooks.html#webhooks-structures) 참조).