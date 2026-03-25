## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## 예제

[inline-code-attrs-start title = 'un_pin_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unpin() -> Result<(), Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-20260325-7a9".to_string(),
        broadcast_id: "news/homepage/launch-article".to_string(),
        sso: Some("sso-jwt-user-0a1b2c3d".to_string()),
    };
    let response: PinComment200Response = un_pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]