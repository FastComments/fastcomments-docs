## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------|
| commentId | string | Tak |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Wywołanie tylko z wymaganym parametrem
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Wywołanie z opcjonalnymi parametrami
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]