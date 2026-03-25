## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Да |  |
| updateComments | string | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9d8f4b2c';
const id: string = 'user_f47ac10b';
const replaceTenantUserBody: ReplaceTenantUserBody = {
  externalId: 'ext-5234',
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['moderator'],
  metadata: { department: 'product', region: 'us-east-1' }
};
const updateComments: string = 'propagate-display-name-to-comments';

const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---