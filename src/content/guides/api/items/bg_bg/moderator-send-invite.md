[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Този маршрут предоставя възможност за покана на единичен `Moderator`.

Съществуват следните ограничения за изпращане на имейл с покана на `Moderator`:
- `Moderator` трябва вече да съществува.
- `fromName` не може да бъде по-дълъг от `100 символа`.

**Забележки:**
- Ако потребител с предоставения имейл вече съществува, той ще бъде поканен да модерира коментарите на вашия tenant.
- Ако потребител с предоставения имейл **не съществува**, връзката за покана ще го насочи през създаването на акаунта им.
- Поканата ще изтече след `30 дни`.

Можем да създадем `Moderator` за потребител, за който знаем само имейла:

[inline-code-attrs-start title = 'Пример за покана на Moderator с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Това ще изпрати имейл като `Bob от TenantName ви кани да бъдете модератор...`

[inline-code-attrs-start title = 'Структура на заявката за покана на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за покана на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
