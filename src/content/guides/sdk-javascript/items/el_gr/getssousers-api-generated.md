## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenantId | string | Ναι |  |
| skip | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // Κλήση χωρίς προαιρετικό `skip`
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // Κλήση με προαιρετικό `skip`
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]