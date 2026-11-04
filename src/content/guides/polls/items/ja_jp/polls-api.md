アンケートはREST APIを使って自分のコードからも管理できます。例えば、バックエンドからコメントにアンケートを追加したり、アンケートを締め切ったり、投票をエクスポートしたりできます:

- [Poll Structure](/guide-api.html#poll-structure) はアンケートオブジェクトとその制限について説明しています。
- [GET](/guide-api.html#poll-get)、[PUT](/guide-api.html#poll-put)、[PATCH](/guide-api.html#poll-patch)、[DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` は、コメント上のアンケートの読み取り、置換、編集、削除を行います。
- [PollVote Structure](/guide-api.html#poll-vote-structure) は個々の投票について説明しています。
- [GET](/guide-api.html#poll-votes-get)、[POST](/guide-api.html#poll-vote-post)、[DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` は、投票の一覧表示、投票の実行、投票の削除を行います。

`poll` を `POST /api/v1/comments` に渡すことで、コメントと一緒にアンケートを作成することもできます。

APIはウィジェットと同じ設定に従います。アンケートが有効になっている必要があり、匿名アンケート投票が許可されていない限り、ログインしていないユーザーによる投票は拒否されます。匿名アンケートの個々の投票はAPIを通じて読み取ることはできません。