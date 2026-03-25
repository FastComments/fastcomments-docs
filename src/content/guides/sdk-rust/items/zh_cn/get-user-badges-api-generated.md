## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| badge_id | String | 否 |  |
| displayed_on_comments | bool | 否 |  |
| limit | f64 | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回：[`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badges_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_badges 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-9876")),
        badge_id: Some(String::from("top-reviewer")),
        displayed_on_comments: Some(true),
        limit: Some(50.0),
        skip: Some(0.0),
    };
    let response: GetUserBadges200Response = get_user_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---