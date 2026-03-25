## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| redirectURL | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'sendLoginLink 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12a9f3b7";
const id: string = "user_84b2c7d1";
const redirectURL: string = "https://app.mycompany.com/welcome?ref=login_email";
const resultWithoutRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
const resultWithRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
[inline-code-end]

---