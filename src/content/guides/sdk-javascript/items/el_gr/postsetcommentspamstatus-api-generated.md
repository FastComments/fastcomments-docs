## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| commentId | string | Ναι |  |
| spam | boolean | Όχι |  |
| permNotSpam | boolean | Όχι |  |
| broadcastId | string | Όχι |  |
| tenantId | string | Όχι |  |
| sso | string | Όχι |  |

## Response

Επιστρέφει: [`PostSetCommentSpamStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentSpamStatusResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα postSetCommentSpamStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSpamStatus(): Promise<void> {
  const commentId: string = "cmt_5f2a1b3c4d6e7f8g9h0i";

  // Μόνο απαιτούμενη παράμετρος
  const resultSimple: PostSetCommentSpamStatusResponse = await postSetCommentSpamStatus(commentId, true);

  // All optional parameters provided
  const resultFull: PostSetCommentSpamStatusResponse = await postSetCommentSpamStatus(
    commentId,
    false,
    true,
    "brd_1234abcd",
    "tenant_42",
    "sso_9876xyz"
  );

  console.log(resultSimple, resultFull);
}
[inline-code-end]