## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Одговор

Враћа: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Пример

[inline-code-attrs-start title = 'Primer getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // Позив без опционо `skip`
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // Позив са опционо `skip`
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]