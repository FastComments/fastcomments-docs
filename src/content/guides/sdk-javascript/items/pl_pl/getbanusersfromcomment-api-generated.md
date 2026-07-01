## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| commentId | string | Tak |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getBanUsersFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Wywołanie ze wszystkimi parametrami
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Wywołanie tylko z wymaganym parametrem
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]