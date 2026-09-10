Webhooks는 REST API를 통해서도 관리할 수 있습니다. 이는 Zapier와 같은 통합이 대시보드를 건드리지 않고도 댓글 이벤트를 구독하는 방식이며, REST Hooks 패턴을 따릅니다: 구독, 이벤트 수신, 구독 해제.

API 구독은 대시보드에 설정된 웹훅과 함께 존재합니다. 댓글 이벤트는 해당 도메인에 대한 대시보드 웹훅과 일치하는 모든 API 구독에 각각 별도의 전달으로 전송됩니다. 이벤트당 구독자 수에 제한이 없습니다.

## 인증

모든 요청에는 `x-api-key` 헤더(또는 `API_KEY` 쿼리 매개변수)에 API 키를, `tenantId` 쿼리 매개변수에 테넌트 ID를 포함해야 합니다. 두 값은 대시보드의 API 비밀 페이지에 표시됩니다.

## 구독

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| 필드 | 예 | 설명 |
|-------|----------|-------------|
| `url` | 예 | 절대적인 http 또는 https URL. |
| `event` | 예 | `comment-created`, `comment-updated` or `comment-deleted`. |
| `domain` | 아니오 | 계정 설정에 있는 도메인. 기본값은 `*`, 이는 모든 도메인의 이벤트를 수신합니다. |
| `method` | 아니오 | `POST` (default), `PUT` or `DELETE`. |

응답에는 구독 정보가 포함됩니다:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

같은 URL을 동일한 이벤트와 도메인에 다시 구독하면 중복을 생성하는 대신 기존 구독을 반환하므로 클라이언트가 안전하게 재시도할 수 있습니다. 각 테넌트는 최대 50개의 API 구독을 가질 수 있습니다.

## 목록

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

테넌트에 대한 모든 웹훅을 반환하며, 대시보드에서 관리되는 웹훅(`"source": "dashboard"`)도 포함됩니다. `event`, `domain` 또는 `source` 로 필터링할 수 있습니다.

## 구독 해제

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

구독을 삭제하면 해당 구독에 대기 중인 모든 이벤트도 삭제됩니다. 이 방법으로 삭제할 수 있는 것은 API를 통해 생성된 구독만 해당됩니다. 대시보드 웹훅은 웹훅 페이지에서 편집합니다.

## 페이로드 및 서명

전달은 대시보드 웹훅과 동일한 페이로드를 사용하며(데이터 구조 참조), 동일한 HMAC 방식으로 서명됩니다(보안 및 API 토큰 참조). API 구독은 레거시 `token` 헤더를 받지 않으므로 대신 `X-FastComments-Signature` 헤더를 검증합니다.

## 410 Gone 응답

API 구독의 엔드포인트가 HTTP `410 Gone` 응답을 반환하면 FastComments는 이를 구독 해제로 처리합니다: 구독이 대기 중인 이벤트와 함께 삭제되며, 이후 전달이 시도되지 않습니다. 대시보드에 설정된 웹훅은 자동으로 삭제되지 않으며, 이 경우 410은 일반적인 실패로 간주됩니다. 다른 모든 실패 상태는 재시도되며 결국 웹훅이 비활성화됩니다. 자세한 내용은 작동 방식 및 재시도 처리에서 설명합니다.

## 대시보드

API 구독은 생성된 도메인 아래의 웹훅 페이지에 나열되며, 관리자는 이를 비활성화, 재활성화 또는 삭제할 수 있습니다.