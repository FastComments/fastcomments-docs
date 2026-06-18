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

[inline-code-attrs-start title = 'unFlagComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a1f';
const commentId: string = 'cmt_20250614_01';
const userId: string = 'user_47d2b9';
const result: FlagComment200Response = await unFlagComment(tenantId, commentId, userId);
[inline-code-end]

---