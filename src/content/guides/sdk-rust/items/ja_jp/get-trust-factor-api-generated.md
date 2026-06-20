## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| user_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_trust_factor_response.rs)

## 例

[inline-code-attrs-start title = 'get_trust_factor の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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