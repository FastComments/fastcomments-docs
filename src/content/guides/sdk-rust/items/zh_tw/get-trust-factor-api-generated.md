## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| user_id | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳：[`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_trust_factor_response.rs)

## 範例

[inline-code-attrs-start title = 'get_trust_factor 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_trust_factor() -> Result<(), Error> {
    let params: GetTrustFactorParams = GetTrustFactorParams {
        user_id: Some(String::from("journalist-984")),
        sso: Some(String::from("google-oauth2|1029384756")),
    };
    let trust_response: GetUserTrustFactorResponse = get_trust_factor(&configuration, params).await?;
    println!("{:#?}", trust_response);
    Ok(())
}
[inline-code-end]

---