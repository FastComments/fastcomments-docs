## Parameters

| 名稱 | 類型 | 必需 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domainToUpdate | string | 是 |  |
| patchDomainConfigParams | PatchDomainConfigParams | 是 |  |

## 回應

回傳: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## 範例

[inline-code-attrs-start title = 'patchDomainConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d9f3c4b";
const domainToUpdate: string = "comments.newsroom.example.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  enabled: true,
  enforceHttps: true, // 已包含可選參數
  allowedOrigins: ["https://newsroom.example.com"] // 已包含可選參數
};
const result: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]