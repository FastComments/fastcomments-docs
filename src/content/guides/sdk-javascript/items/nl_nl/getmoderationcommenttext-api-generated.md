## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| commentId | string | Ja |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Example

[inline-code-attrs-start title = 'Voorbeeld van getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Roep aan met alleen de verplichte parameter
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Roep aan met optionele parameters
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]