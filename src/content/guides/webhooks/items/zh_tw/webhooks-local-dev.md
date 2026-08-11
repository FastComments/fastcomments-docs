For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### 步驟 1：在您的帳戶中新增「localhost」至網域。

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='帳戶設定中新增網域的表單，已在網域名稱欄位輸入 localhost'; title='新增 localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 步驟 2：選擇 API 金鑰

We're going to be adding webhook configuration for your domain, so we'll need an API key. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='新的 API 密鑰表單，已將關聯的網域設定為 localhost，金鑰名稱為 Testing'; title='新增 Testing API 金鑰'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**注意：或者，您可以為所有測試活動和預備環境使用同一個 API 密鑰。只需為「所有網域」新增一個 API 密鑰，並給它一個像「test」的名稱。**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### 步驟 3：新增您的 Webhook

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhook 管理介面，已選取 localhost 網域，並在「評論建立」端點填入 ngrok URL'; title='新增 Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### 步驟 4：新增評論

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay
for the events to reach your machine.