## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| comment_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_response.rs)

## 例

[inline-code-attrs-start title = 'post_remove_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_remove_comment() -> Result<PostRemoveCommentResponse, Error> {
    let params: PostRemoveCommentParams = PostRemoveCommentParams {
        comment_id: String::from("cmt-9f8b6a3"),
        sso: Some(String::from("sso-token-6f4e9a2b")),
    };
    let response: PostRemoveCommentResponse = post_remove_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]