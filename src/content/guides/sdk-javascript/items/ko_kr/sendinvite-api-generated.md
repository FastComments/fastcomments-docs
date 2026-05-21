## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'sendInvite 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-128';
const id: string = 'comment-8421f';
const fromName: string = 'Marcus Lindström';
const note: string | undefined = undefined; // 선택적 매개변수 예시
const response: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---