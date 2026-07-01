## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odgovor

Vrne: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Primer

[inline-code-attrs-start title = 'getModerationCommentText Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Pokličite samo z obveznim parametrom
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Pokličite z neobveznimi parametri
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]