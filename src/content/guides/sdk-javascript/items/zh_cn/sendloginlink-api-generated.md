---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| redirectURL | string | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'sendLoginLink 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async function run(): Promise<void> {
  const tenantId: string = 'fc_tenant_9f3b2c';
  const id: string = 'user_42b7f';
  const redirectURL: string = 'https://dashboard.acme-corp.com/welcome';
  const responseWithoutRedirect: APIEmptyResponse = await sendLoginLink(tenantId, id);
  const responseWithRedirect: APIEmptyResponse = await sendLoginLink(tenantId, id, redirectURL);
  console.log(responseWithoutRedirect, responseWithRedirect);
})();
[inline-code-end]

---