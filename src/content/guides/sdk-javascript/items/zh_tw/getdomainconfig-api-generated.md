## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## 回應

返回：[`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigResponse.ts)

## 範例

[inline-code-attrs-start title = 'getDomainConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main() {
  const tenantId: string = "acme-corp-123";
  const domain: string = "blog.acme.com";
  const config: GetDomainConfigResponse = await getDomainConfig(tenantId, domain);
  console.log(config);
}
main();
[inline-code-end]