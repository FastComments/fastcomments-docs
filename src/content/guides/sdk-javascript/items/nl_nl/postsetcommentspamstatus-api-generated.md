## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| commentId | string | Yes |  |
| spam | boolean | No |  |
| permNotSpam | boolean | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Response

Retourneert: [`PostSetCommentSpamStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentSpamStatusResponse.ts)

## Example

[inline-code-attrs-start title = 'postSetCommentSpamStatus Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSpamStatus(): Promise<void> {
  const commentId: string = "cmt_5f2a1b3c4d6e7f8g9h0i";

  // Alleen benodigde parameter
  const resultSimple: PostSetCommentSpamStatusResponse = await postSetCommentSpamStatus(commentId, true);

  // Alle optionele parameters opgegeven
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

---