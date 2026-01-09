[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава ажурирање једног SSO корисника.

[inline-code-attrs-start title = 'SSOUser ажурирање cURL примјер'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

У овом примјеру наводимо `groupIds` за контролу приступа, али ово је опционално.

[inline-code-attrs-start title = 'SSOUser Структура захтјева за ажурирање'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Када се промијени имејл или корисничко име, овај параметар можете поставити на true да би се такође ажурирали коментари корисника. Ово ће удвостручити трошкове кредита. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Структура одговора за ажурирање'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    user?: SSOUser; // Враћамо ажурираног корисника у случају успјеха.
}
[inline-code-end]


---