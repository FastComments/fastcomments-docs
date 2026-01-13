[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

此 API 端点提供创建电子邮件模板的能力。

注意：

- 在相同域名下，不能有多个相同 `emailTemplateId` 的模板。
- 但你可以有一个通配符模板（`domain` = `*`）和同一 `emailTemplateId` 的域特定模板共存。
- 指定 `domain` 仅在你有不同域名，或想为测试使用特定模板（例如将 `domain` 设置为 `localhost`）时才相关。
- 如果你确实指定了 `domain`，它必须匹配一个 `DomainConfig`。发生错误时会提供一个有效域名列表。
- 模板语法为 EJS，渲染有 500ms 的超时。渲染的 P99 小于 5ms，因此如果达到 500ms 就说明有问题。
- **你的模板必须能使用提供的 `testData` 成功渲染** 才能保存。渲染错误会被聚合并在仪表盘上报告（很快也会通过 API 提供）。

添加模板所需的最少数据如下：

[inline-code-attrs-start title = '最小 EmailTemplate POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

你可能希望为每个站点设置模板，在这种情况下你可以定义 `domain`：

[inline-code-attrs-start title = 'EmailTemplate POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'EmailTemplate POST 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** 失败时包含的原因。 **/
    reason?: string
    /** 已创建的模板。 **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]