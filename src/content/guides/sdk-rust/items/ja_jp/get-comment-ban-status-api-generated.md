## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| comment_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_ban_status_response.rs)

## 例

[inline-code-attrs-start title = 'get_comment_ban_status の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetCommentBanStatusParams = GetCommentBanStatusParams {
        comment_id: String::from("cmt-9f8b7a6e-4d3c-11ee-8c99-0242ac120002"),
        sso: Some(String::from("acme-corp-tenant")),
    };
    let response: GetCommentBanStatusResponse = get_comment_ban_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---