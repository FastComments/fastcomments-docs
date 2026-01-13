[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

此 API 端點提供建立電子郵件範本的功能。

注意事項：

- 相同網域下不能有多個使用相同 `emailTemplateId` 的範本。
- 但你可以同時擁有通配符範本（`domain` = `*`）以及同一 `emailTemplateId` 的特定網域範本。
- 只有在你擁有不同網域，或想針對特定情境使用專屬範本（例如測試時將 `domain` 設為 `localhost`）時，才需要指定 `domain`。
- 如果指定 `domain`，它必須符合某個 `DomainConfig`。若錯誤，會提供可使用的網域清單。
- 範本語法為 EJS，渲染有 500ms 的逾時限制。渲染的 P99 小於 5ms，因此若達到 500ms 表示有問題。
- **要儲存，範本必須能使用你提供的 `testData` 成功渲染**。渲染錯誤會彙總並顯示在儀表板（很快也將透過 API 提供）。 

新增範本所需的最少資料如下：

[inline-code-attrs-start title = '最小 EmailTemplate POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

你可能想為每個網站設定範本，在這種情況下請定義 `domain`：

[inline-code-attrs-start title = 'EmailTemplate POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** 失敗時會包含。 **/
    reason?: string
    /** 已建立的範本。 **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---