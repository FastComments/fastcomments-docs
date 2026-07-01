## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`GetCommentChildrenResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentChildrenResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentChildren voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const tenantId: string = "tenant_xyz";
const sso: string = "sso_987654";

const fullResponse: GetCommentChildrenResponse = await getCommentChildren(commentId, tenantId, sso);
const minimalResponse: GetCommentChildrenResponse = await getCommentChildren(commentId);
[inline-code-end]

---