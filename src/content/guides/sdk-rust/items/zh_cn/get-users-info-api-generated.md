---
租户的批量用户信息。根据 userIds，返回 User / SSOUser 的显示信息。评论部件使用此接口为通过存在事件刚出现的用户提供丰富信息。没有页面上下文：隐私统一强制执行（私人资料会被遮蔽）。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## 响应

返回：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## 示例

[inline-code-attrs-start title = '获取用户信息 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---