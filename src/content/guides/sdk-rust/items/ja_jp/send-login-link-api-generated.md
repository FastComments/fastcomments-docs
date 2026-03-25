## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| redirect_url | String | いいえ |  |

## レスポンス

返却: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 例

[inline-code-attrs-start title = 'send_login_link の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_login_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("user-98765"),
        redirect_url: Some(String::from("https://acme.example.com/dashboard")),
    };
    let response: FlagCommentPublic200Response = send_login_link(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---