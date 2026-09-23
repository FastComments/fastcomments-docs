## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| commentId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserInternalProfile'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const ssoToken: string = "sso_abcde12345";

  const profileOnly: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId);
  const profileWithComment: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId);
  const fullProfile: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId, ssoToken);
}

demo();
[inline-code-end]

---