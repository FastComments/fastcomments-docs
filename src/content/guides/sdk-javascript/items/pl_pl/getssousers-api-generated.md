## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Odpowiedź

Zwraca: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // Wywołanie bez opcjonalnego `skip`
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // Wywołanie z opcjonalnym `skip`
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]