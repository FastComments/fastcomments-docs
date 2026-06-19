## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| editKey | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Dönen Değer: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentText Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_prod_01';
const commentId: string = 'cmt_5f2d9b8a-3e7c-4a1b';
const editKey: string = 'edit_8b3f6c2d4a9e';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const response: PublicAPIGetCommentTextResponse = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]

---