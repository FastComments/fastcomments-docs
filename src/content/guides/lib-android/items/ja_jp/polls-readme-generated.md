投票はウェブウィジェットと同じように動作し、FastCommentsダッシュボードの同じウィジェット設定から有効化できます(または上記の `pollCreation` / `pollVoting` を使用)。

- 投票付きのコメントは、テキストの下に投票を表示します。読者は選択肢をタップして投票でき、投票前に結果の表示/非表示を切り替えることができ、投票の変更も可能です。他の人が投票すると集計はリアルタイムで更新されます。
- 投票は日付で締め切ることができ、投票するまで結果を非表示にでき、投票者を管理者または全員に表示させる("See who voted")こともできます。
- 閲覧者が投票を作成できる場合、`FastCommentsView` のコメント入力欄に投票ビルダーを開く投票ボタンが表示されます。ツールバーを有効にしたことがなくても、このボタンのためにツールバー行が表示されます。`sdk.setCommentToolbarEnabled(false)` を呼び出すと投票ボタンを含めて非表示になり、入力欄の `setPollCreationEnabled(false)` は投票ボタンのみを削除します。`setCommentToolbarEnabled` は `FastCommentsView` を作成する前に呼び出してください。入力欄はSDKを受け取った時点でこの設定を読み取るためです。すでに画面上にあるビューの場合は、代わりに `getBottomCommentInput().setToolbarVisible(false)` を呼び出してください。ライブチャットの入力欄は、ウェブのライブチャットと同様にこれを表示しません。

独自のUIから投票を投稿するには、`CommentPollInput` を `postComment` に渡します:

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

`sdk.votePoll(commentId, optionId, callback)` と `sdk.getPollVoters(commentId, page, search, callback)` で投票と投票者リストを扱えます。投票の色はTHEMING.mdに記載されています。