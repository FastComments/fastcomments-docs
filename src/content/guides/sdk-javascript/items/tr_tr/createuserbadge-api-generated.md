## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createUserBadgeParams | CreateUserBadgeParams | Evet |  |

## Yanıt

Döndürür: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APICreateUserBadgeResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createUserBadge Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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