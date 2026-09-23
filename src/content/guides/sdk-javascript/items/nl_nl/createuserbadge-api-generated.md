## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createUserBadgeParams | CreateUserBadgeParams | Ja |  |

## Respons

Retourneert: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APICreateUserBadgeResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createUserBadge voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c";
  const badgeParams: CreateUserBadgeParams = {
    name: "Community Helper",
    iconUrl: "https://cdn.example.com/badges/helper.png"
  };
  const response: APICreateUserBadgeResponse = await createUserBadge(tenantId, badgeParams);
}
main();
[inline-code-end]

---