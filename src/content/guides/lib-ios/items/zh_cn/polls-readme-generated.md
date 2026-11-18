投票功能与 Web 小部件中的工作方式相同，可通过 FastComments 仪表板中相同的小部件设置开启：`pollCreation`（谁可以附加投票：禁用、管理员和版主，或所有人）和 `pollVoting`（已登录用户或任何人）。

- 带有投票的评论会在其文本下方显示投票。读者通过点击选项进行投票，可以在投票前显示或隐藏结果，并且可以更改投票。随着其他人投票，票数会实时更新。
- 投票可以设置截止日期，在投票前隐藏结果，并让投票者对管理员或所有人可见（"查看谁投票了"）。
- 当查看者可以创建投票时，`FastCommentsView` 的评论输入框会显示一个投票按钮，用于打开投票构建器。投票随评论一起发布，因此评论文本是必填项。实时聊天输入框不包含该按钮，与 Web 实时聊天一致。

投票需要已登录的用户，因此请使用 [SSO](#authentication-sso-readme-generated)。服务器将匿名投票绑定到会话 cookie，而 API 客户端不会保留该 cookie，因此当 `pollVoting` 设置为任何人时，没有 SSO 用户的查看者会看到"登录以投票"。在投票前隐藏结果的投票不会获得实时票数更新；这些投票者将在下次加载时看到新数字。

要从您自己的 UI 发布投票，请将 `CommentPollInput` 传递给 `postComment`：

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` 和 `sdk.getPollVoters(commentId:page:search:)` 涵盖投票和投票者列表。投票颜色列在 [投票颜色](#theming-readme-generated) 下。