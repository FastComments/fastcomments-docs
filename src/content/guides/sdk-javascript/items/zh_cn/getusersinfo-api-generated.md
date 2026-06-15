为租户批量获取用户信息。给定 userIds，返回 User / SSOUser 的展示信息。
由评论小部件使用，用于丰富那些通过在线状态事件刚刚出现的用户信息。
无页面上下文：隐私统一强制执行（私人档案将被屏蔽）。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 是 |  |

## 响应

返回：[`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // 可选；如果为 undefined 则默认使用逗号
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]