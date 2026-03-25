## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tag | string | Da |  |
| tenantId | string | Ne |  |
| deleteHashTagRequest | DeleteHashTagRequest | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "breaking-news";
const tenantId: string = "tenant_12345";
const deleteRequest: DeleteHashTagRequest = { initiatedBy: "moderator@newsorg.com", purgeAllOccurrences: true };
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteRequest);
[inline-code-end]

---