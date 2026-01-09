[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Этот маршрут предоставляет возможность пригласить одного `Moderator`.

Существуют следующие ограничения для отправки электронного приглашения `Moderator`:
- `Moderator` должен уже существовать.
- `fromName` не может быть длиннее `100 characters`.

**Примечания:**
- Если пользователь с указанным адресом электронной почты уже существует, ему будет отправлено приглашение модерировать комментарии вашего тенанта.
- Если пользователь с указанным адресом электронной почты **не существует**, ссылка-приглашение проведет его через процесс создания аккаунта.
- Срок действия приглашения истечет через `30 days`.

Мы можем создать `Moderator` для пользователя, о котором известен только адрес электронной почты:

[inline-code-attrs-start title = 'Пример cURL-запроса приглашения модератора'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Это отправит письмо вроде `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура запроса приглашения модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** Электронное письмо, отправленное пользователю, будет выглядеть как отправленное от этого имени. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа на приглашение модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]