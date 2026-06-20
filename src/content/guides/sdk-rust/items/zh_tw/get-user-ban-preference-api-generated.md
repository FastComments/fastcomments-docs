## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| sso | String | 否 |  |

## 回應

回傳: [`ApiModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_moderate_get_user_ban_preferences_response.rs)

## 範例

[inline-code-attrs-start title = 'get_user_ban_preference 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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