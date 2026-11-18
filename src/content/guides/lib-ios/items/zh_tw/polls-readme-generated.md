投票功能與網頁小工具中的運作方式相同，可從 FastComments 儀表板中相同的小工具設定開啟:`pollCreation`(誰可以附加投票:停用、管理員與版主,或所有人)和 `pollVoting`(已登入的使用者或任何人)。

- 附帶投票的留言會在其文字下方顯示投票。讀者透過點擊選項來投票,可以在投票前顯示或隱藏結果,並可以更改投票。當其他人投票時,票數會即時更新。
- 投票可以設定在某個日期關閉、在你投票前隱藏結果,並讓投票者對管理員或所有人可見(「查看誰投票了」)。
- 當檢視者可以建立投票時,`FastCommentsView` 的留言輸入框會顯示一個投票按鈕,開啟投票建立器。投票會隨留言一起發佈,因此留言文字為必填。即時聊天輸入框不會顯示該按鈕,與網頁版即時聊天相同。

投票需要已登入的使用者,因此請使用 [SSO](#authentication-sso-readme-generated)。伺服器會將匿名投票綁定到 session cookie,而 API 用戶端不會保留該 cookie,因此當 `pollVoting` 設定為任何人時,沒有 SSO 使用者的檢視者會看到「登入以投票」。在你投票前隱藏結果的投票不會有即時票數更新;這些投票者會在下一次載入時看到新的數字。

若要從你自己的 UI 發佈投票,請將 `CommentPollInput` 傳遞給 `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` 和 `sdk.getPollVoters(commentId:page:search:)` 涵蓋投票與投票者名單。投票顏色列於 [Poll Colors](#theming-readme-generated) 之下。