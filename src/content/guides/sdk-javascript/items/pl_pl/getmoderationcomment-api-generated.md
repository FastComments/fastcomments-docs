## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Pełny zestaw parametrów
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // Minimalne wywołanie używające tylko wymaganego argumentu
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Użyj wyników w razie potrzeby...
}
[inline-code-end]