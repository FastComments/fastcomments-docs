[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Этот маршрут позволяет отправить ссылку для входа одному `TenantUser`.

Полезно при массовом создании пользователей, чтобы не объяснять им, как войти на FastComments.com. Это просто отправит им «магическую ссылку» для входа, срок действия которой истекает через `30 days`.

Существуют следующие ограничения для отправки ссылки входа `TenantUser`:
- `TenantUser` должен уже существовать.
- У вас должен быть доступ к управлению `Tenant`, которому принадлежит `TenantUser`.

Мы можем отправить ссылку для входа `TenantUser` следующим образом:

[inline-code-attrs-start title = 'Пример cURL-запроса ссылки входа TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

This will send an email like `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура запроса ссылки входа TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа ссылки входа TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]