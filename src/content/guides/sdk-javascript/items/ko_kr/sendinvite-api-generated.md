## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## 응답

반환: [`SendInviteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendInviteResponse.ts)

## 예시

[inline-code-attrs-start title = 'sendInvite 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const inviteId: string = "invite-12345";
const fromName: string = "John Doe";

const inviteResult: SendInviteResponse = await sendInvite(tenantId, inviteId, fromName);
[inline-code-end]