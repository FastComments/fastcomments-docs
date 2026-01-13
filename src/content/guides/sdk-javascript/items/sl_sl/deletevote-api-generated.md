## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| editKey | string | Ne |  |

## Odgovor

Vrne: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b21c9';
const id: string = 'vote_4a2d9f1b';
const editKey: string = 'edit_92b7c6a1';

const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
[inline-code-end]