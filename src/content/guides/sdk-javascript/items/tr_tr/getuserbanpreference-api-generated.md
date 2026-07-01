## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetUserBanPreferenceResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBanPreferenceResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUserBanPreference Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBanPreference(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-2024";
  const sso: string = "sso-token-9f8b7a6c";

  const result: GetUserBanPreferenceResponse = await getUserBanPreference(tenantId, sso);
  console.log(result);
}

demoGetUserBanPreference();
[inline-code-end]

---