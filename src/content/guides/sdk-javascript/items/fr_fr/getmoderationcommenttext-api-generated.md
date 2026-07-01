## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Exemple

[inline-code-attrs-start title = 'getModerationCommentText Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Appel avec le seul paramètre requis
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Appel avec des paramètres optionnels
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]