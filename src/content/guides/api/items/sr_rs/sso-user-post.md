[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава креирање једног SSO корисника.

Покушај креирања два корисника са истим ID-јем ће резултирати грешком.

[inline-code-attrs-start title = 'SSOUser креирање — пример cURL захтева'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

У овом примеру наводимо `groupIds` за контролу приступа, али ово је опционално.

[inline-code-attrs-start title = 'SSOUser Структура захтева за креирање'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Структура одговора за креирање'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Укључено у случају неуспеха. **/
    reason?: string
    user?: SSOUser; // Враћамо креираног корисника у случају успеха.
}
[inline-code-end]

#### Напомена о интеграцији

Подаци прослеђени преко API-ја могу се преписати једноставним слањем другог HMAC payload-а за SSO корисника. На пример, ако поставите username преко API-ја, али затим пошаљете други преко SSO тока при учитавању странице, ми ћемо аутоматски ажурирати њихов username.

Нећемо ажурирати параметре корисника у овом току осим ако их изричито не наведете или не подесите на null (не undefined).