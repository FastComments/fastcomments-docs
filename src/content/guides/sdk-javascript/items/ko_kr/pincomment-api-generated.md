## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'pinComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f2b9a";
const commentId: string = "cmt_9f8e7d6c";
const broadcastId: string = "brd_live_concert_2026-03-25";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload_signature";

const result: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---