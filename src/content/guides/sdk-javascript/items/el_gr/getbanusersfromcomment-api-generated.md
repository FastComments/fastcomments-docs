## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| commentId | string | Ναι |  |
| tenantId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getBanUsersFromComment Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Κλήση με όλες τις παραμέτρους
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Κλήση μόνο με την απαιτούμενη παράμετρο
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]