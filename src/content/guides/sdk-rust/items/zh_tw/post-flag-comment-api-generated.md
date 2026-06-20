## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'post_flag_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PostFlagCommentParams = PostFlagCommentParams {
        comment_id: String::from("news/acme-corp/article-237/comment-8421"),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.acme-sso-payload")),
    };
    let response: ApiEmptyResponse = post_flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---