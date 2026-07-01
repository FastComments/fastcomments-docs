## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| limit | f64 | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回: [`ApiGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_list_response.rs)

## 示例

[inline-code-attrs-start title = '获取用户徽章进度列表 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge_progress(conf: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-98765".to_string()),
        limit: Some(20.0),
        skip: Some(5.0),
    };
    let _resp = get_user_badge_progress_list(conf, params).await?;
    Ok(())
}
[inline-code-end]