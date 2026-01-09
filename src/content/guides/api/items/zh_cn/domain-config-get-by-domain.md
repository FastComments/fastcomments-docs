[api-resource-header-start name = 'DomainConfig'; route = 'GET /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

单个 DomainConfigs 可通过其对应的 `domain` 获取。 

[inline-code-attrs-start title = '按域名的域配置 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '按域名的域配置请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigsByDomainRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '按域名的域配置响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-domain' | 'update-would-create-duplicate' | 'domain-does-not-exist'
    /** 失败时包含。 **/
    reason?: string
    configuration?: DomainConfig | null
}
[inline-code-end]

---