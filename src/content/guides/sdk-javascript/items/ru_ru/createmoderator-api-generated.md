## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createModeratorBody | CreateModeratorBody | Да |  |

## Ответ

Возвращает: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // необязательный параметр (демонстрация)
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---