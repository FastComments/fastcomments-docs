ローカル開発には、[ngrok](https://ngrok.com/) のようなツールを使用してください。

システムのセキュリティを維持しやすくするため、ローカル開発は他の環境のセットアップおよび保護と同じ手順に従います。

### ステップ 1: アカウントのドメインに "localhost" を追加する

"localhost" を[こちらでドメインとして追加](https://fastcomments.com/auth/my-account/configure-domains)してください。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='アカウント設定のドメイン追加フォームに、ドメイン名フィールドへ localhost が入力されている様子'; title='localhost を追加'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### ステップ 2: API キーを選択する

ドメインの Webhook 設定を追加するため、API キーが必要です。[こちらから追加できます。](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='関連ドメインが localhost に設定され、キー名が Testing と入力された新規 API シークレットフォーム'; title='Testing API キーを追加'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

「Associate with domain(ドメインとの関連付け)」で、"localhost" ドメインを選択してください。

**注意: または、すべてのテスト作業とステージング環境に対して 1 つの API シークレットを使用することもできます。その場合は「All Domains(すべてのドメイン)」に対して API シークレットを追加し、「test」のような名前を付けてください。**

本番ドメイン用の API シークレットが定義されていることを確認してください。その他のすべてのドメインのイベントには、ワイルドカード(テスト用)シークレットが使用されます。

### ステップ 3: Webhook を追加する

ngrok または同様のツールを実行している状態で、"localhost" の値を[こちら](https://fastcomments.com/auth/my-account/manage-data/webhooks)で設定してください。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='localhost ドメインが選択され、コメント作成エンドポイントに ngrok の URL が入力された Webhooks 管理画面'; title='テスト用 Webhook を追加'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

`Send Test Payload` をクリックすると、API キーの検証が正しく行われているかを確認するため、2 つのテストイベントが送信されます。

検証が成功したら、`Save` を押してください。

### ステップ 4: コメントを追加する

これで、コメントの追加、編集、削除が可能になり、テスト用 API キーを使用して、イベントがローカル開発マシンに送信されるのを確認できるはずです。イベントがマシンに到達するまで、最大 30 秒ほどかかる場合があります。