Webhooks 관리자에는 각 이벤트 유형 (Create, Update, Delete)에 대해 `Send Test Payload` 버튼이 있습니다. Create 및 Update 이벤트는 더미 WebhookComment 객체를 전송하며, Delete 테스트는 ID만 있는 더미 요청 본문을 전송합니다.

## 페이로드 검증

웹훅 통합을 테스트할 때, 들어오는 요청에 다음 헤더들이 포함되어 있는지 확인하세요:

1. **`token`** - 귀하의 API 비밀
2. **`X-FastComments-Timestamp`** - Unix 타임스탬프(초)
3. **`X-FastComments-Signature`** - HMAC-SHA256 서명

HMAC 서명 검증을 사용하여 페이로드의 진위를 확인하세요.

## 테스트 도구

개발 중 들어오는 웹훅 페이로드를 검사하기 위해 [webhook.site](https://webhook.site) 또는 [ngrok](https://ngrok.com)과 같은 도구를 사용할 수 있습니다.

## 이벤트 유형

- **Create Event**: 새 댓글이 생성될 때 트리거됩니다. 기본 메서드: PUT
- **Update Event**: 댓글이 편집될 때 트리거됩니다. 기본 메서드: PUT
- **Delete Event**: 댓글이 삭제될 때 트리거됩니다. 기본 메서드: DELETE

각 이벤트는 요청 본문에 전체 댓글 데이터를 포함합니다 (페이로드 형식은 [데이터 구조](/guides/webhooks/webhooks-structures) 참조).