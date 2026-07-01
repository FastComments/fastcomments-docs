## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| postIds | Array<string> | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserReactsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublicResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserReactsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const postIds: string[] = ["post_1a2b3c", "post_4d5e6f"];
  const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

  const fullResponse: GetUserReactsPublicResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
  const minimalResponse: GetUserReactsPublicResponse = await getUserReactsPublic(tenantId);
}

demo();
[inline-code-end]

---