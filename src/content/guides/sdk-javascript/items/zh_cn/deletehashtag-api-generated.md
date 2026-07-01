## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tag | string | 是 |  |
| tenantId | string | 否 |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | 否 |  |

## 响应

返回: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteHashTag 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tag: string = "announcement";
  const tenantId: string = "tenant_9876";
  const requestBody: DeleteHashTagRequestBody = {
    confirmDeletion: true
  };
  const response: DeleteHashTagResponse = await deleteHashTag(tag, tenantId, requestBody);
  console.log(response);
})();
[inline-code-end]