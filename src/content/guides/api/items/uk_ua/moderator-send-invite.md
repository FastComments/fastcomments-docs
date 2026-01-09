[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Цей маршрут надає можливість запросити одного `Moderator`.

Існують такі обмеження для відправки запрошення електронною поштою `Moderator`:
- `Moderator` повинен уже існувати.
- `fromName` не може бути довшим за `100 characters`.

**Примітки:**
- Якщо користувач із вказаною електронною поштою вже існує, йому буде надіслано запрошення модерувати коментарі вашого тенанта.
- Якщо користувача з вказаною електронною поштою **не існує**, посилання-запрошення проведе його через процес створення облікового запису.
- Термін дії запрошення закінчується через `30 days`.

Ми можемо створити `Moderator` для користувача, про якого ми знаємо лише електронну пошту:

[inline-code-attrs-start title = 'Приклад cURL-запиту для запрошення модератора'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Це надішле електронний лист приблизно такого змісту: `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура запиту для запрошення модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** Електронний лист, надісланий користувачу, буде виглядати так, ніби він надійшов від цього імені. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на запрошення модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Включено у разі помилки. **/
    reason?: string
}
[inline-code-end]

---