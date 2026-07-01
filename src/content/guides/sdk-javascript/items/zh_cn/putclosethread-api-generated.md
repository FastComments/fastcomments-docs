## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| urlId | string | 是 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`PutCloseThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutCloseThreadResponse.ts)

## 示例

[inline-code-attrs-start title = 'putCloseThread 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function closeThreadDemo(): Promise<void> {
  const urlId: string = "article-2023-09-15";
  const tenantId: string = "tenant-42";
  const sso: string = "sso-token-xyz";

  const response: PutCloseThreadResponse = await putCloseThread(urlId, tenantId, sso);
  console.log(response);
}

closeThreadDemo();
[inline-code-end]