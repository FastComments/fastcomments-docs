租户的批量用户信息。给定 userIds，返回来自 User / SSOUser 的显示信息。
由评论小部件使用，用于丰富那些通过 presence 事件刚出现的用户。
无页面上下文：隐私统一强制执行（私人资料会被屏蔽）。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## 响应

返回：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo 只需要 tenantId 和 ids；可选参数在这里不适用。
[inline-code-end]

---