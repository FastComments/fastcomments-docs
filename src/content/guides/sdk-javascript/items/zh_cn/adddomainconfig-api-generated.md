## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| addDomainConfigParams | AddDomainConfigParams | Yes |  |

## 响应

返回: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## 示例

[inline-code-attrs-start title = 'addDomainConfig 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const config: AddDomainConfigParams = {
    domain: 'myblog.example.com',
    enabled: true,
    // 描述是可选的，此处省略
  };
  const response: AddDomainConfigResponse = await addDomainConfig(tenantId, config);
  console.log(response);
})();
[inline-code-end]