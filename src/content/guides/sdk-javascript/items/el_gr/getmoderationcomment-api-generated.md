## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| includeEmail | boolean | Όχι |  |
| includeIP | boolean | Όχι |  |
| sso | string | Όχι |  |

## Response

Επιστρέφει: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Example

[inline-code-attrs-start title = 'getModerationComment Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Κλήση μόνο με απαιτούμενες παραμέτρους
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // Κλήση με προαιρετικές παραμέτρους
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]

---