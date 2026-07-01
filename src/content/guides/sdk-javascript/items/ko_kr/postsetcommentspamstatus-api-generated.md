## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| spam | boolean | No |  |
| permNotSpam | boolean | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

Returns: [`PostSetCommentSpamStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentSpamStatusResponse.ts)

## 예제

[inline-code-attrs-start title = 'postSetCommentSpamStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSpamStatus(): Promise<void> {
  const commentId: string = "cmt_5f2a1b3c4d6e7f8g9h0i";

  // 필수 매개변수만
  const resultSimple: PostSetCommentSpamStatusResponse = await postSetCommentSpamStatus(commentId, true);

  // 모든 선택적 매개변수가 제공됨
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