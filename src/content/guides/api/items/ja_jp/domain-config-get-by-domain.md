[api-resource-header-start name = 'DomainConfig'; route = 'GET /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

個々の DomainConfigs は対応する `domain` によって取得できます。 

[inline-code-attrs-start title = 'ドメインによる Domain Config の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'ドメインによる Domain Config リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigsByDomainRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ドメインによる Domain Config レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-domain' | 'update-would-create-duplicate' | 'domain-does-not-exist'
    /** 失敗時に含まれます。 **/
    reason?: string
    configuration?: DomainConfig | null
}
[inline-code-end]

---