## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Réponse

Renvoie : [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Appeler uniquement avec le paramètre requis
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Appeler avec les paramètres optionnels
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]

---