投票功能與網頁小工具中的運作方式相同，可透過 FastComments 儀表板中的相同小工具設定開啟(或使用上方的 `pollCreation` / `pollVoting`)。

- 帶有投票的留言會在其文字下方顯示投票。讀者可透過點擊選項來投票,可在投票前顯示或隱藏結果,並可更改投票。當其他人投票時,票數會即時更新。
- 投票可以設定截止日期、在您投票前隱藏結果,並讓投票者對管理員或所有人可見("查看誰投票了")。
- 當檢視者可以建立投票時,`FastCommentsView` 的留言輸入框會顯示一個投票按鈕,點擊後會開啟投票建立器。即使您從未開啟工具列,工具列列也會為此顯示;`sdk.setCommentToolbarEnabled(false)` 會隱藏它(包括投票按鈕),而輸入框上的 `setPollCreationEnabled(false)` 則只會移除投票按鈕。請在建立 `FastCommentsView` 之前呼叫 `setCommentToolbarEnabled`,因為輸入框在接收 SDK 時會讀取該設定;對於已顯示在畫面上的檢視,請改為呼叫 `getBottomCommentInput().setToolbarVisible(false)`。即時聊天輸入框不包含此功能,與網頁版即時聊天相同。

若要從您自己的 UI 發布投票,請將 `CommentPollInput` 傳遞給 `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optional: an ISO 8601 close date, PollPrivacy value, and results hidden until you vote.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` 和 `sdk.getPollVoters(commentId, page, search, callback)` 涵蓋投票功能與投票者名單。投票顏色列於 THEMING.md 中。