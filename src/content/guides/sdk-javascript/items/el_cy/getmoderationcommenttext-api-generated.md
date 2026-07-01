## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| commentId | string | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Απόκριση

Επιστρέφει: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Κλήση μόνο με την απαιτούμενη παράμετρο
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Κλήση με προαιρετικές παραμέτρους
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]