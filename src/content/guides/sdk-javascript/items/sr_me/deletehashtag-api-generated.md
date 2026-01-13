## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tag | string | Da |  |
| tenantId | string | Ne |  |
| deleteHashTagRequest | DeleteHashTagRequest | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer za deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = 'breaking-news';
const tenantId: string = 'tenant_42';
const deleteReq: DeleteHashTagRequest = { removedBy: 'moderator_jane', reason: 'off-topic for this community', deleteAssociatedComments: true } as DeleteHashTagRequest;
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteReq);
[inline-code-end]

---