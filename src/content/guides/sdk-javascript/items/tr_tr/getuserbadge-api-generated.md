## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUserBadge Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-001";
  const badgeId: string = "badge-5f9d3a2b";

  const badgeResponse: GetUserBadgeResponse = await getUserBadge(tenantId, badgeId);

  // Opsiyonel alanlara güvenli bir şekilde erişin
  const badgeName: string | undefined = badgeResponse.userBadge?.name;
  console.log(`Badge ID: ${badgeId}, Name: ${badgeName ?? "Unnamed"}`);
}

runExample();
[inline-code-end]