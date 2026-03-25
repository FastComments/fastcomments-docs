## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tag | string | Sì |  |
| tenantId | string | No |  |
| deleteHashTagRequest | DeleteHashTagRequest | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "breaking-news";
const tenantId: string = "tenant_12345";
const deleteRequest: DeleteHashTagRequest = { initiatedBy: "moderator@newsorg.com", purgeAllOccurrences: true };
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteRequest);
[inline-code-end]