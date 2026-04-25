---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deletePendingWebhookEvent 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b2a";
const webhookEventId: string = "wh_evt_9a8c7d1234";
const dryRun: boolean | undefined = undefined; // 선택적 플래그 예제 (이 호출에서 필요하지 않음)
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, webhookEventId);
[inline-code-end]

---