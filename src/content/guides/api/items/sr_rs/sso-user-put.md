[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава ажурирање појединачног SSO корисника.

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање SSO корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

У овом примеру наводимо `groupIds` за контролу приступа, али ово је опционално.

[inline-code-attrs-start title = 'Структура захтева за ажурирање SSO корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Када се мења имејл или корисничко име, можете поставити ово на true да бисте такође ажурирали коментаре корисника. Ово ће удвостручити трошкове у кредитима. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за ажурирање SSO корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Укључено у случају неуспеха. **/
    reason?: string
    user?: SSOUser; // Враћамо ажурираног корисника у случају успеха.
}
[inline-code-end]