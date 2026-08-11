---
本番環境と同様に `localhost` に対しても同じ手順を実行してください。プロダクションのドメインと API シークレットが設定されていることを確認してください。

まず、[Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks) に移動します。これは Manage Data -> Webhooks からアクセスできます。

設定ページは以下のように表示されます：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='コメントイベントごとにドメインセレクタとエンドポイント URL フィールドがあり、Send Test Payload ボタンがある Webhooks 管理ページ'; title='Webhooks 設定'; cacheBuster = 'v3' app-screenshot-end]

このページでは、各種コメントイベントごとにエンドポイントを指定できます。

各イベントタイプについて、統合が正しく設定されていることを確認するために必ず Send Test Payload をクリックしてください。詳細は次のセクション「Testing」を参照してください。