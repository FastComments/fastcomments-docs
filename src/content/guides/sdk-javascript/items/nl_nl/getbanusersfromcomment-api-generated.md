## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| commentId | string | Ja |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getBanUsersFromComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Aanroep met alle parameters
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Aanroep met alleen de verplichte parameter
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]