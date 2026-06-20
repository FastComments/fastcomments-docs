---
## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| sso | String | 否 |  |

## 响应

返回：[`ApiModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_moderate_get_user_ban_preferences_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_ban_preference 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserBanPreferenceParams = GetUserBanPreferenceParams {
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ApiModerateGetUserBanPreferencesResponse =
        get_user_ban_preference(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---