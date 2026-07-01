## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| includeEmail | boolean | Nee |  |
| includeIP | boolean | Nee |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getModerationComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Volledig parameterset
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

  // Minimale aanroep met alleen het vereiste argument
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Gebruik resultaten naar behoefte...
}
[inline-code-end]