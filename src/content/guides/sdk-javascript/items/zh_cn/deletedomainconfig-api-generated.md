## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domain | string | 是 |  |

## 响应

返回: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteDomainConfig 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'acme-corp';
  const domain: string = 'blog.acme.com';
  const response: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
  console.log(response);
}
runExample();
[inline-code-end]

---