[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Этот маршрут позволяет отправить ссылку для входа одному `TenantUser`.

Полезно при массовом создании пользователей, чтобы не объяснять им, как войти на FastComments.com. Это просто отправит им «магическую ссылку» для входа, которая истекает через `30 days`.

Следующие ограничения действуют при отправке ссылки для входа `TenantUser`:
- `TenantUser` должен уже существовать.
- Вы должны иметь доступ к управлению `Tenant`, к которому принадлежит `TenantUser`.

Мы можем отправить ссылку для входа `TenantUser` следующим образом:

[inline-code-attrs-start title = 'Пример cURL-запроса ссылки для входа TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Это отправит электронное письмо вида `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура запроса ссылки входа TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа запроса ссылки входа TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Указывается при неудаче. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Указывается при неудаче. **/
    reason?: string
}
[inline-code-end]

---