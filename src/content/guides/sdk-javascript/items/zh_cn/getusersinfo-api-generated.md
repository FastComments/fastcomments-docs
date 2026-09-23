租户的大批量用户信息。给定 userIds，返回来自 User / SSOUser 的显示信息。  
由评论小部件使用，以在用户通过存在事件刚出现时丰富其信息。  
无页面上下文：隐私统一强制执行（私人资料被屏蔽）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| ids | string | 是 |  |

## 响应

返回：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---