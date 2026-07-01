## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Returned: [`GetCommentChildrenResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentChildrenResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getCommentChildren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const tenantId: string = "tenant_xyz";
const sso: string = "sso_987654";

const fullResponse: GetCommentChildrenResponse = await getCommentChildren(commentId, tenantId, sso);
const minimalResponse: GetCommentChildrenResponse = await getCommentChildren(commentId);
[inline-code-end]