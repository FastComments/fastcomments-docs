## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | String | はい |  |
| tenant_id | String | いいえ |  |
| delete_hash_tag_request | models::DeleteHashTagRequest | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 例

[inline-code-attrs-start title = 'delete_hash_tag の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteHashTagParams = DeleteHashTagParams {
        tag: "news/politics".to_string(),
        tenant_id: Some("acme-corp-tenant".to_string()),
        delete_hash_tag_request: Some(models::DeleteHashTagRequest::default()),
    };
    let response: FlagCommentPublic200Response = delete_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---