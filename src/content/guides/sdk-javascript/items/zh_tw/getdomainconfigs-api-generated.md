## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 回應

返回: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getDomainConfigs 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDomainConfigs(): Promise<void> {
  const tenantId: string = "acme-corp-567";
  const configs: GetDomainConfigsResponse = await getDomainConfigs(tenantId);
  console.log(configs);
}
[inline-code-end]