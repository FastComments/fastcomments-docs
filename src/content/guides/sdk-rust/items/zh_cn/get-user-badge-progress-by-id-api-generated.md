## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_badge_progress_by_id 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp".to_string(),
        id: "user-12345".to_string(),
    };
    let _response = get_user_badge_progress_by_id(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---