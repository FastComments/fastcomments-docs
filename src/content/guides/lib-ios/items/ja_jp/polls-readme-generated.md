投票はウェブウィジェットと同様に機能し、FastCommentsダッシュボードの同じウィジェット設定から有効化します: `pollCreation`(誰が投票を添付できるか: 無効、管理者とモデレーター、または全員)と `pollVoting`(ログインユーザーまたは誰でも)。

- 投票が添付されたコメントは、テキストの下に投票を表示します。読者は選択肢をタップして投票でき、投票前に結果の表示/非表示を切り替えられ、投票の変更も可能です。他の人が投票すると集計はリアルタイムで更新されます。
- 投票は日付で締め切ることができ、投票するまで結果を非表示にしたり、投票者を管理者または全員に公開する("See who voted")設定も可能です。
- 閲覧者が投票を作成できる場合、`FastCommentsView` のコメント入力に投票ボタンが表示され、タップすると投票ビルダーが開きます。投票はコメントと一緒に投稿されるため、コメントテキストが必須です。ライブチャットの入力では、ウェブのライブチャットと同様にこのボタンは表示されません。

投票にはログインユーザーが必要なため、[SSO](#authentication-sso-readme-generated) を使用してください。サーバーは匿名の投票をセッションCookieに紐付けますが、APIクライアントはセッションCookieを保持しないため、`pollVoting` が「誰でも」に設定されている場合、SSOユーザーのいない閲覧者には「Log in to vote」と表示されます。投票するまで結果を非表示にする投票では、リアルタイムの集計更新は行われず、その投票者は次回の読み込み時に新しい数値を確認できます。

独自のUIから投票を投稿するには、`CommentPollInput` を `postComment` に渡します:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` と `sdk.getPollVoters(commentId:page:search:)` で投票と投票者リストを扱えます。投票の色は [Poll Colors](#theming-readme-generated) に記載されています。