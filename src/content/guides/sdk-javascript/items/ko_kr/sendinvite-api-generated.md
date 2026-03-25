## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| fromName | string | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'sendInvite 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-42';
const id: string = 'comment_8f3b21a7';
const fromName: string = 'Elena Morales';
const replyToEmail: string | undefined = undefined;

const result: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName, replyToEmail);
[inline-code-end]

---