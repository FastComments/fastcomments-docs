[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

您可能會注意到，可以使用租戶 ID 為 "demo" 的評論小工具，例如：

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

這僅供您試用評論小工具。在生產環境中，您會像下面這樣傳遞您的租戶 ID：

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

您的租戶 ID 已套用在評論小工具的 <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">程式碼片段</a> 中，您可以在那裡找到。

您也可以在 [API 憑證頁面](https://fastcomments.com/auth/my-account/api-secret) 找到您的租戶 ID 並管理您的 API 金鑰。

從此之後，如果您已登入 FastComments，程式碼範例將會使用您的實際租戶 ID（如果您是在 https://fastcomments.com 登入的話）。