---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'blockFromCommentPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6b3f9a2d';
const commentId: string = 'cmt_8f4b12a9';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'Repeated promotional links',
  durationMinutes: 60 * 24 * 30, // 30 dagen
  escalateToModeration: true
};
const sso: string = 'sso_token_3fH7kLw';

const result: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---