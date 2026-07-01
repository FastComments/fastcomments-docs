## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Response

Zwraca: [`GetUserBanPreferenceResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBanPreferenceResponse.ts)

## Example

[inline-code-attrs-start title = 'Przykład getUserBanPreference'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBanPreference(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-2024";
  const sso: string = "sso-token-9f8b7a6c";

  const result: GetUserBanPreferenceResponse = await getUserBanPreference(tenantId, sso);
  console.log(result);
}

demoGetUserBanPreference();
[inline-code-end]