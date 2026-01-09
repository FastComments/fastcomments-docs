[api-resource-header-start name = 'DomainConfig'; route = 'PUT /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントはドメイン構成を置き換える機能を提供します。

[inline-code-attrs-start title = 'DomainConfig PUT cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"domain": "new-domain-example.com",
	"emailFromName": "some from name",
	"emailFromEmail": "some@test.com",
	"logoSrc": "https://example.com/my-logo-big.png",
	"logoSrc100px": "https://example.com/my-logo-100px.png",
	"footerUnsubscribeURL": "http://example.com/unsubscribe-ui",
	"emailHeaders": {
		"List-Unsubscribe-Post": "List-Unsubscribe=One-Click",
		"List-Unsubscribe": "<https://example.com/opt-out/[userId]>"
	}
}'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PUT リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPutQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PUT レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPutResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'domain-does-not-exist' | 'update-would-create-duplicate' | 'domain-too-long' | 'domain-invalid';  
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 更新された構成。 **/
    configuration?: DomainConfig
}
[inline-code-end]

---