本番と同じ手順を `localhost` に対して行ってください。本番ドメインと API Secrets が設定されていることを確認してください。

まず、[Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks) に移動してください。これは Manage Data -> Webhooks からアクセスできます。

設定ページは次のように表示されます：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

このページで、各種コメントイベントごとのエンドポイントを指定できます。

各イベントタイプごとに、統合が正しく設定されていることを確認するために必ず「Send Test Payload」をクリックしてください。詳しくは次のセクション「Testing」を参照してください。

---