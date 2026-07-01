## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| commentId | string | Ja |  |
| spam | boolean | Nein |  |
| permNotSpam | boolean | Nein |  |
| broadcastId | string | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`PostSetCommentSpamStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentSpamStatusResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentSpamStatus Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSpamStatus(): Promise<void> {
  const commentId: string = "cmt_5f2a1b3c4d6e7f8g9h0i";

  // Nur erforderlicher Parameter
  const resultSimple: PostSetCommentSpamStatusResponse = await postSetCommentSpamStatus(commentId, true);

  // Alle optionalen Parameter bereitgestellt
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