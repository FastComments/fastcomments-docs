## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| sure | string | Не |  |

## Одговор

Враћа: [`DeleteTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteTenant Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "tenant_12345";
    const id: string = "resource_98765";
    const sure: string = "confirm";

    // Позив са опционом 'sure' параметром
    const responseWithSure: DeleteTenantResponse = await deleteTenant(tenantId, id, sure);
    console.log(responseWithSure);

    // Позив без опционо 'sure' параметра
    const responseWithoutSure: DeleteTenantResponse = await deleteTenant(tenantId, id);
    console.log(responseWithoutSure);
}

runExample();
[inline-code-end]