## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|---------------|-------------|
| commentId | string | Sì |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Chiamata con solo il parametro obbligatorio
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Chiamata con parametri opzionali
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]