For Local development, use a tool like [ngrok](https://ngrok.com/).

ローカル開発には、[ngrok](https://ngrok.com/) のようなツールを使用してください。

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

システムのセキュリティを簡素化するために、ローカル開発は他の環境のセットアップおよび保護と同じプロセスに従います。

### Step 1: Add "localhost" to domains in your account.

### ステップ 1: アカウントのドメインに "localhost" を追加する

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

"localhost" を [こちら](https://fastcomments.com/auth/my-account/configure-domains) でドメインとして追加してください。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

### ステップ 2: API Key を選択する

We're going to be adding webhook configuration for your domain, so we'll need an API key. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

ドメイン用にウェブフックの設定を追加するため、API Key が必要です。[こちらで作成できます。](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

"Associate with domain" の項目で、"localhost" ドメインを選択してください。

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

**注意: 代替として、すべてのテスト活動およびステージング環境に対して1つの API Secret を使用することもできます。単に "All Domains" 用の API Secret を追加し、名前を "test" のように付けてください。**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

本番ドメインには必ず API Secret を定義してください。その他のすべてのドメインのイベントはワイルドカード（テスト）シークレットを使用します。

### Step 3: Add Your Webhook

### ステップ 3: ウェブフックを追加する

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

ngrok 等のツールを実行している間に、"localhost" の値を [こちら](https://fastcomments.com/auth/my-account/manage-data/webhooks) で設定してください。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

`Send Test Payload` をクリックすると、API Key の検証を確認するために 2 件のテストイベントを送信します。

Once it validates, hit `Save`.

検証が完了したら、`Save` を押してください。

### Step 4: Add A Comment

### ステップ 4: コメントを追加する

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay
for the events to reach your machine.

これでコメントの追加、編集、削除が可能になり、テスト用 API Key を使用してイベントでローカル開発マシンにコールが来るのが確認できるはずです。イベントがマシンに届くまでに最大で 30 秒の遅延が発生する場合があります。