## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| fromName | string | Evet |  |

## Yanıt

Döndürür: [`SendInviteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendInviteResponse.ts)

## Örnek

[inline-code-attrs-start title = 'sendInvite Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const inviteId: string = "invite-12345";
const fromName: string = "John Doe";

const inviteResult: SendInviteResponse = await sendInvite(tenantId, inviteId, fromName);
[inline-code-end]

---