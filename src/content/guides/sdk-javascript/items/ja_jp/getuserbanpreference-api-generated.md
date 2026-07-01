## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`GetUserBanPreferenceResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBanPreferenceResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserBanPreference の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBanPreference(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-2024";
  const sso: string = "sso-token-9f8b7a6c";

  const result: GetUserBanPreferenceResponse = await getUserBanPreference(tenantId, sso);
  console.log(result);
}

demoGetUserBanPreference();
[inline-code-end]