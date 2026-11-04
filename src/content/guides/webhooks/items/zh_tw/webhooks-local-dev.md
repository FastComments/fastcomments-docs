若要進行本機開發，請使用像 [ngrok](https://ngrok.com/) 這樣的工具。

為了簡化系統安全性的維護，本機開發遵循與設定及保護其他環境相同的流程。

### 步驟 1：將 "localhost" 新增至您帳戶的網域中。

[在此處](https://fastcomments.com/auth/my-account/configure-domains)將 "localhost" 新增為網域。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='帳戶設定中的新增網域表單，網域名稱欄位中已輸入 localhost'; title='新增 localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 步驟 2：選擇一個 API 金鑰

我們將為您的網域新增 webhook 設定，因此需要一個 API 金鑰。[您可以在此處進行設定。](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='新的 API 密鑰表單，關聯網域設為 localhost，金鑰命名為 Testing'; title='新增 Testing API 金鑰'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

在「Associate with domain」下方 - 選擇您的「localhost」網域。

**注意：或者，您可以為所有測試活動和預備環境使用同一個 API 密鑰。只需為「All Domains」新增一個 API 密鑰，並將其命名為「test」之類的名稱。**

請確保您已為正式環境網域定義了 API 密鑰。所有其他網域的事件將使用萬用字元（測試）密鑰。

### 步驟 3：新增您的 Webhook

在執行 ngrok 或類似工具時，[在此處](https://fastcomments.com/auth/my-account/manage-data/webhooks)為「localhost」設定值。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhooks 管理介面，已選取 localhost 網域，並在留言建立端點中填入 ngrok URL'; title='新增測試 Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

當點擊 `Send Test Payload` 時，我們會傳送兩個測試事件，以檢查您是否驗證了 API 金鑰。

驗證通過後，點擊 `Save`。

### 步驟 4：新增一則留言

現在您可以新增、編輯或刪除留言，並應該會看到我們使用您的測試 API 金鑰，將事件呼叫傳送至您的本機開發機器。事件可能需要最多 30 秒的延遲才會到達您的機器。