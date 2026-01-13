## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateModeratorBody | UpdateModeratorBody | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '4f8a9c2e-3b6d-4d9e-8c2f-1a2b3c4d5e6f';
const id: string = 'mod_92a7c4';
const updateModeratorBodyMinimal: UpdateModeratorBody = { displayName: 'Sophia Patel' };
const updateModeratorBodyWithOptional: UpdateModeratorBody = {
  displayName: 'Sophia Patel',
  email: 'sophia.patel@newsroom.example',
  permissions: ['remove_comments', 'ban_user'],
  notifyOnFlag: true // демонстрација опционалног параметра
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBodyWithOptional);
[inline-code-end]

---