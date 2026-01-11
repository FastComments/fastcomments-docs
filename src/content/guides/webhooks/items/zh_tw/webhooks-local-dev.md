在本機開發時，可使用像 [ngrok](https://ngrok.com/) 之類的工具。

為了簡化系統安全維護，本機開發遵循與設定及保護其他環境相同的流程。 

### 步驟 1：在您的帳戶網域中加入「localhost」。

在 [此處](https://fastcomments.com/auth/my-account/configure-domains) 將「localhost」新增為網域。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 步驟 2：選擇 API 金鑰

我們會為您的網域新增 webhook 設定，因此需要一個 API 金鑰。您可以在 [此處](https://fastcomments.com/auth/my-account/api-secret) 設定。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

在「Associate with domain」下方，選擇您的「localhost」網域。

**注意：或者，您可以為所有測試活動和預備環境使用同一個 API Secret。只要為「All Domains」新增一個 API Secret，並命名為例如「test」。**

請確保為您的正式環境網域定義了 API Secret。所有其他網域的事件將使用通配（測試）金鑰。

### 步驟 3：新增您的 Webhook

在執行 ngrok 或類似工具時，請在 [此處](https://fastcomments.com/auth/my-account/manage-data/webhooks) 為「localhost」設定值。

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

驗證通過後，按下 `Save`。

### 步驟 4：新增一則評論

現在您可以新增、編輯或刪除評論，應該會看到我們使用您的測試 API 金鑰，以事件呼叫您的本機開發機器。可能會有最多 30 秒的延遲
事件才會到達您的機器。

---