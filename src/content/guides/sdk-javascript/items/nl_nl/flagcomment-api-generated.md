## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Respons

Retourneert: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'flagComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const id: string = 'comment_7f3a2b9e';
const userId: string = 'user_jdoe_1001';
const anonUserId: string = 'anon_3f2b_visitor';
const result: FlagComment200Response = await flagComment(tenantId, id, userId, anonUserId);
[inline-code-end]

---