## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| fromName | string | Ja |  |

## Antwoord

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'sendInvite Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-42';
const id: string = 'comment_8f3b21a7';
const fromName: string = 'Elena Morales';
const replyToEmail: string | undefined = undefined;

const result: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName, replyToEmail);
[inline-code-end]

---