## Parameter

| Name | Type | Required | Beschreibung |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sure | string | Nein |  |

## Antwort

Rückgabe: [`DeleteTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "tenant_12345";
    const id: string = "resource_98765";
    const sure: string = "confirm";

    // Aufruf mit optionalem 'sure'-Parameter
    const responseWithSure: DeleteTenantResponse = await deleteTenant(tenantId, id, sure);
    console.log(responseWithSure);

    // Aufruf ohne optionalen 'sure'-Parameter
    const responseWithoutSure: DeleteTenantResponse = await deleteTenant(tenantId, id);
    console.log(responseWithoutSure);
}

runExample();
[inline-code-end]