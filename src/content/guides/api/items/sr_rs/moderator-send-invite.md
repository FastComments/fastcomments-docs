[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ова рута омогућава да позовете једног `Moderator`.

Постоје следећа ограничења за слање позива путем е-поште `Moderator`-у:
- `Moderator` већ мора постојати.
- `fromName` не сме бити дужи од `100 characters`.

**Напомене:**
- Ако корисник са датом е-поштом већ постоји, биће позван да модерира коментаре вашег тенанта.
- Ако корисник са датом е-поштом **не постоји**, линк за позив ће их провести кроз креирање налога.
- Позив ће истећи након `30 days`.

Можемо креирати `Moderator` за корисника за кога знамо само е-пошту:

[inline-code-attrs-start title = 'Пример cURL захтева за позив модератора'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ово ће послати е-поруку као `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура захтева за позив модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** Е-порука послата кориснику ће изгледати као да је послата са овог имена. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за позив модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]