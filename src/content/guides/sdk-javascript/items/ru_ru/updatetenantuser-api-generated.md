## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateTenantUserBody | UpdateTenantUserBody | Да |  |
| updateComments | string | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const userId: string = 'u_72b9f4';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['moderator'],
  suspended: false
};
const updateComments: string = 'Promoted to moderator after review of activity and community feedback';
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, userId, updateTenantUserBody, updateComments);
[inline-code-end]

---