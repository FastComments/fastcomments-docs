## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| commentId | string | Ja |  |
| includeEmail | boolean | Nein |  |
| includeIP | boolean | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getModerationComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Vollständiger Parametersatz
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

  // Minimaler Aufruf nur mit dem erforderlichen Argument
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Ergebnisse bei Bedarf verwenden...
}
[inline-code-end]