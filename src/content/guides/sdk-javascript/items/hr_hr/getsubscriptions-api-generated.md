## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Odgovor

Vraća: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getSubscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const userId: string = "user-789";

    const responseWithUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
    const responseWithoutUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
})();
[inline-code-end]

---