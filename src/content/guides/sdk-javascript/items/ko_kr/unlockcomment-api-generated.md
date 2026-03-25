## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'unLockComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9d4f2b';
const commentId: string = 'cmt_8a3e1f';
const broadcastId: string = 'broadcast_2026_03_25';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';

const result: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---