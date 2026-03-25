## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_badge_progress_by_id 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_badge_progress() -> Result<GetUserBadgeProgressById200Response, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-007-community-builder".to_string(),
    };
    let response: GetUserBadgeProgressById200Response = get_user_badge_progress_by_id(cfg, params).await?;
    Ok(response)
}
[inline-code-end]

---