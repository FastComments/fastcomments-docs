## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sendEmail | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteModerator 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2d4a6c';
const moderatorId: string = 'moderator_84a1b9c2';
const sendEmail: string = 'true';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);
[inline-code-end]

---