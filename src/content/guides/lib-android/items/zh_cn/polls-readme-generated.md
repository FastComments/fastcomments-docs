投票功能与 Web 挂件中的工作方式相同，可在 FastComments 控制面板的相同挂件设置中开启（或通过上面的 `pollCreation` / `pollVoting` 开启）。

- 带投票的评论会在其文本下方显示投票。读者通过点击选项进行投票，投票前可以显示或隐藏结果，并且可以更改投票。随着其他人投票，计票结果会实时更新。
- 投票可以设置截止日期、在你投票前隐藏结果，并使投票者对管理员或所有人可见（“查看谁投了票”）。
- 当查看者可以创建投票时，`FastCommentsView` 的评论输入框会显示一个投票按钮，用于打开投票构建器。即使你从未开启工具栏，工具栏行也会为此显示；`sdk.setCommentToolbarEnabled(false)` 会将其隐藏（包括投票按钮），而输入框上的 `setPollCreationEnabled(false)` 仅移除投票按钮。请在创建 `FastCommentsView` 之前调用 `setCommentToolbarEnabled`，因为输入框在接收 SDK 时会读取该设置；对于已在屏幕上的视图，请改为调用 `getBottomCommentInput().setToolbarVisible(false)`。实时聊天输入框不包含此功能，与 Web 端实时聊天一致。

要从你自己的 UI 发布投票，请将 `CommentPollInput` 传递给 `postComment`：

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // 可选：ISO 8601 截止日期、PollPrivacy 值，以及投票前隐藏结果。
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` 和 `sdk.getPollVoters(commentId, page, search, callback)` 涵盖投票和投票者列表。投票颜色已在 THEMING.md 中列出。