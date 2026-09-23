單一投票的個別投票，最舊的排在前面。

投票屬於一則評論，因此投票總是一次讀取單一投票 - 必須提供 commentId。這樣可確保所有查詢都使用集合已建立的索引。

遵守投票的隱私設定：匿名投票的投票無法被讀取（poll-anonymous），無論在此或透過 ID。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## 回應

回傳：[`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## 範例

[inline-code-attrs-start title = '取得投票結果 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]

---