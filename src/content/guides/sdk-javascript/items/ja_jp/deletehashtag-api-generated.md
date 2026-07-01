## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | No |  |

## レスポンス

返却: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteHashTag の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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