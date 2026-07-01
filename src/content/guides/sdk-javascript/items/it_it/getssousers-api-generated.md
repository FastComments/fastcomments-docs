## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // Chiamata senza `skip` opzionale
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // Chiamata con `skip` opzionale
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]

---