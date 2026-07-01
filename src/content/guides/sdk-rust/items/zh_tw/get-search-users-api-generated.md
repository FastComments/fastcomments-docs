## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| value | String | 否 |  |
| sso | String | 否 |  |

## 回應

返回: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_user_search_response.rs)

## 範例

[inline-code-attrs-start title = 'get_search_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("john.doe".to_string()),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ModerationUserSearchResponse = get_search_users(configuration, params).await?;
    Ok(())
}
[inline-code-end]