## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| deleteComments | boolean | 否 |  |
| commentDeleteMode | string | 否 |  |

## 响应

返回：[`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSSOUserAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteSSOUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const userId: string = "sso-user-42";
const deleteComments: boolean = true;
const commentDeleteMode: string = "hard";

const detailedResult: DeleteSSOUserAPIResponse = await deleteSSOUser(
  tenantId,
  userId,
  deleteComments,
  commentDeleteMode
);

const simpleResult: DeleteSSOUserAPIResponse = await deleteSSOUser(
  tenantId,
  userId
);
[inline-code-end]