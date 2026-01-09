[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ta API končna točka omogoča ustvarjanje `Subscription`. Upoštevajte, da ima lahko uporabnik samo eno naročnino na stran, saj je več odvečno, in poskus
ustvariti več kot eno naročnino za istega uporabnika na isti strani bo povzročil napako.

Ustvarjanje naročnine bo povzročilo ustvarjanje objektov `Notification`, ko je na korenu naročenega `urlId` objavljen nov komentar (ko je `parentId` komentarja `null`).

[inline-code-attrs-start title = 'Primer cURL POST za Subscription'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST zahtevka za Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST odgovora za Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]