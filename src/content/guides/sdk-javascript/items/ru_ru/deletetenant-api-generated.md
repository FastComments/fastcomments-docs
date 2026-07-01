## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sure | string | No |  |

## Ответ

Возвращает: [`DeleteTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "tenant_12345";
    const id: string = "resource_98765";
    const sure: string = "confirm";

    // Вызов с необязательным параметром 'sure'
    const responseWithSure: DeleteTenantResponse = await deleteTenant(tenantId, id, sure);
    console.log(responseWithSure);

    // Вызов без необязательного параметра 'sure'
    const responseWithoutSure: DeleteTenantResponse = await deleteTenant(tenantId, id);
    console.log(responseWithoutSure);
}

runExample();
[inline-code-end]