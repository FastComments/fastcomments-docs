---
针对租户的大批量用户信息。给定 userIds，返回来自 User / SSOUser 的显示信息。由评论小部件使用，以丰富通过存在事件刚出现的用户。没有页面上下文：隐私被统一强制执行（私有个人资料会被屏蔽）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## 响应

返回: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]

---