---
租户的批量用户信息。给定 userIds，返回 User / SSOUser 的显示信息。
由评论小部件使用，用于丰富通过 presence event 刚刚出现的用户。
没有页面上下文：隐私被统一强制执行（私有资料被掩盖）。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| ids | String | 是 |  |

## 响应

返回: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## 示例

[inline-code-attrs-start title = 'get_users_info 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---