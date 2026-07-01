## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| sure | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`DeleteTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "tenant_12345";
    const id: string = "resource_98765";
    const sure: string = "confirm";

    // Κλήση με προαιρετική παράμετρο 'sure'
    const responseWithSure: DeleteTenantResponse = await deleteTenant(tenantId, id, sure);
    console.log(responseWithSure);

    // Κλήση χωρίς προαιρετική παράμετρο 'sure'
    const responseWithoutSure: DeleteTenantResponse = await deleteTenant(tenantId, id);
    console.log(responseWithoutSure);
}

runExample();
[inline-code-end]