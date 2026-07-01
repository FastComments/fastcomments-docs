## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| commentId | string | Da |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getModerationCommentText Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Poziv samo s obaveznim parametrom
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Poziv s opcionim parametrima
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]