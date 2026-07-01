## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| redirectURL | string | Non |  |

## Réponse

Renvoie : [`SendLoginLinkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendLoginLinkResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple sendLoginLink'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSendLoginLink() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const redirectURL: string = "https://app.acme-corp.com/auth/callback";

  const resultWithRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const resultWithoutRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId);
}
[inline-code-end]