[api-resource-header-start name = 'DomainConfig'; route = 'POST /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Овај API крајња тачка омогућава креирање конфигурација домена.

Додавање конфигурације за домен овлашћава тај домен за FastComments налог.

Уобичајени случајеви употребе овог API-ја су почетна подешавања, када је потребно додати више домена, или прилагођена конфигурација за слање имејлова. 

[inline-code-attrs-start title = 'Пример cURL захтева за DomainConfig POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"domain": "example.com",
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

[inline-code-attrs-start title = 'Структура POST захтева за DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура POST одговора за DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'configuration-exists-for-domain' | 'domain-too-long' | 'domain-invalid';  
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Креирана конфигурација. **/
    configuration?: DomainConfig
}
[inline-code-end]

---