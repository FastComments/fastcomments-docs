## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'blockFromCommentPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6b3f9a2d';
const commentId: string = 'cmt_8f4b12a9';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'Repeated promotional links',
  durationMinutes: 60 * 24 * 30, // 30 gün
  escalateToModeration: true
};
const sso: string = 'sso_token_3fH7kLw';

const result: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---