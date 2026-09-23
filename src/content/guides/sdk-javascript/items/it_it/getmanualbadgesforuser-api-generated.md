## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| badgesUserId | string | No |  |
| commentId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserManualBadgesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getManualBadgesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const badgesUserId: string | undefined = "user_98765";
  const commentId: string | undefined = "comment_abcde";
  const sso: string | undefined = "sso_token_xyz";

  const basicResponse: GetUserManualBadgesResponse = await getManualBadgesForUser(tenantId);
  const fullResponse: GetUserManualBadgesResponse = await getManualBadgesForUser(
    tenantId,
    badgesUserId,
    commentId,
    sso
  );

  console.log(basicResponse, fullResponse);
}();
[inline-code-end]

---