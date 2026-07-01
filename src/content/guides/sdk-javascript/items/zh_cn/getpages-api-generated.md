## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 响应

返回：[`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'getPages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

(async () => {
  const response: GetPagesAPIResponse = await getPages(tenantId);
  const firstPage: APIPage | undefined = response.pages?.[0];
  console.log(firstPage?.title);
})();
[inline-code-end]