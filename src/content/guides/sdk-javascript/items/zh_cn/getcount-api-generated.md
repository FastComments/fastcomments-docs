## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filter | string | No |  |
| searchFilters | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 响应

返回：[`GetCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountResponse.ts)

## 示例

[inline-code-attrs-start title = 'getCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const count: GetCountResponse = await getCount({
    textSearch: "order issue",
    byIPFromComment: "198.51.100.23",
    filter: "pending",
    demo: true,
    tenantId: "acme_corp",
    sso: "sso_abcdef123456"
  });
  console.log(count);
}
main();
[inline-code-end]