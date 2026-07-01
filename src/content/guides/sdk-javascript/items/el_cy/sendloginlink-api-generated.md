## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| redirectURL | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`SendLoginLinkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendLoginLinkResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'sendLoginLink Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSendLoginLink() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const redirectURL: string = "https://app.acme-corp.com/auth/callback";

  const resultWithRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const resultWithoutRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId);
}
[inline-code-end]