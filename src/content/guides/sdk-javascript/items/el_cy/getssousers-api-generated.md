## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Απόκριση

Επιστρέφει: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // Κλήση χωρίς το προαιρετικό `skip`
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // Κλήση με το προαιρετικό `skip`
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]