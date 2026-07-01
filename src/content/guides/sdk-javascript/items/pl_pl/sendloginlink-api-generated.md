## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| redirectURL | string | Nie |  |

## Odpowiedź

Zwraca: [`SendLoginLinkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendLoginLinkResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład sendLoginLink'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSendLoginLink() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const redirectURL: string = "https://app.acme-corp.com/auth/callback";

  const resultWithRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const resultWithoutRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId);
}
[inline-code-end]