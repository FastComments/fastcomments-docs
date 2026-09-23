## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| sso | string | No |  |

## Yanıt

Döndürür: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getModerationComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Yalnızca gerekli parametrelerle çağır
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // İsteğe bağlı parametrelerle çağır
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]