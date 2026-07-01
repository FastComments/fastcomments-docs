## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Απάντηση

Returns: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Συνολικό σύνολο παραμέτρων
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // Ελάχιστη κλήση χρησιμοποιώντας μόνο το απαιτούμενο όρισμα
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Χρησιμοποιήστε τα αποτελέσματα όπως χρειάζεται...
}
[inline-code-end]