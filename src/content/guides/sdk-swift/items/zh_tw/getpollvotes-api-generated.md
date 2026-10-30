The individual votes behind one poll's tallies, oldest first.

每個投票的個別記錄，依投票時間由舊到新排列。

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

投票屬於某則評論，因此投票資料總是一次讀取單一投票 - 必須提供 commentId。這樣可確保所有查詢皆使用集合已建立的索引。

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

遵守投票隱私設定：匿名投票的投票結果無法被讀取（poll-anonymous），無論在此或透過 ID。

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 是 |  |
| voterId | string | query | 否 |  |
| optionId | string | query | 否 |  |
| skip | integer | query | 否 |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPollVotes 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬於測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (可選)
let optionId = "optionId_example" // String |  (可選)
let skip = 987 // Int |  (可選)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---