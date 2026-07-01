## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateModeratorBody | UpdateModeratorBody | Yes |  |

## Ответ

Возвращает: [`UpdateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateModeratorResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUpdateModerator(): Promise<void> {
    const tenantId: string = "tenant_42abc";
    const moderatorId: string = "moderator_8f9e";
    const updateBody: UpdateModeratorBody = {
        isActive: true,
        role: "admin",
        // необязательное поле
        notes: "Promoted to senior moderator"
    };
    const result: UpdateModeratorResponse = await updateModerator(tenantId, moderatorId, updateBody);
    console.log(result);
}

demoUpdateModerator();
[inline-code-end]