[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава креирање једног SSO корисника.

Покушај креирања два корисника са истим ID-ом резултираће грешком.

[inline-code-attrs-start title = 'Примјер cURL захтјева за SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтјева за креирање SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за креирање SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    user?: SSOUser; // Враћамо креираног корисника у случају успјеха.
}
[inline-code-end]

#### Напомена о интеграцији

Подаци прослијеђени преко API-ја могу се преписати једноставним слањем другачијег SSO User HMAC payload-а. На примјер, ако
подесите корисничко име преко API-ја, али потом пошаљете друго корисничко име преко SSO тока при учитавању странице, ми ћемо аутоматски ажурирати
њихово корисничко име.

Нећемо ажурирати параметре корисника у овом току осим ако их јасно не назначите или не подесите на null (не undefined).