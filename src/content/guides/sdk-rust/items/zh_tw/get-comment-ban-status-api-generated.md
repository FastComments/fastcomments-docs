## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳： [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_ban_status_response.rs)

## 範例

[inline-code-attrs-start title = 'get_comment_ban_status 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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