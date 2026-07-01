## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentChildrenResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentChildrenResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentChildren Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const tenantId: string = "tenant_xyz";
const sso: string = "sso_987654";

const fullResponse: GetCommentChildrenResponse = await getCommentChildren(commentId, tenantId, sso);
const minimalResponse: GetCommentChildrenResponse = await getCommentChildren(commentId);
[inline-code-end]