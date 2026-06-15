## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'unFlagComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a1f';
const commentId: string = 'cmt_20250614_01';
const userId: string = 'user_47d2b9';
const result: FlagComment200Response = await unFlagComment(tenantId, commentId, userId);
[inline-code-end]

---