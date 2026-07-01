## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## 回應

返回：[`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## 範例

[inline-code-attrs-start title = 'deleteDomainConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'acme-corp';
  const domain: string = 'blog.acme.com';
  const response: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
  console.log(response);
}
runExample();
[inline-code-end]