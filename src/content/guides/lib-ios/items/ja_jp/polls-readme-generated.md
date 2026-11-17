アンケート（Poll）はWebウィジェットと同様に動作し、FastCommentsダッシュボードの同じウィジェット設定から有効化できます: `pollCreation`（アンケートを添付できる対象: 無効、管理者とモデレーター、または全員）と `pollVoting`（ログインユーザーまたは誰でも）。

- アンケート付きのコメントは、テキストの下にアンケートを表示します。読者は選択肢をタップして投票でき、投票前に結果の表示/非表示を切り替えることができ、投票の変更も可能です。他の人が投票すると集計はリアルタイムで更新されます。
- アンケートは日付で締め切ることができ、投票するまで結果を非表示にしたり、投票者を管理者または全員に公開する（「投票者を見る」）設定も可能です。
- 閲覧者がアンケートを作成できる場合、`FastCommentsView` のコメント入力欄にアンケートボタンが表示され、アンケートビルダーが開きます。アンケートはコメントと一緒に投稿されるため、コメントテキストが必須です。ライブチャットの入力欄には、Webのライブチャットと同様にこのボタンは表示されません。

投票にはログインユーザーが必要なため、[SSO](#authentication-sso-readme-generated) を使用してください。サーバーは匿名の投票をセッションCookieに紐付けますが、APIクライアントはセッションCookieを保持しないため、`pollVoting` が「誰でも」に設定されている場合、SSOユーザーのいない閲覧者には「投票するにはログインしてください」と表示されます。投票するまで結果を非表示にするアンケートでは、リアルタイムの集計更新は行われず、その投票者には次回の読み込み時に新しい数値が表示されます。

独自のUIからアンケートを投稿するには、`CommentPollInput` を `postComment` に渡します:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` と `sdk.getPollVoters(commentId:page:search:)` で、投票と投票者リストに対応しています。アンケートの色は [Poll Colors](#poll-colors-readme-generated) に記載されています。