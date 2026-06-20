---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| spam | bool | 否 |  |
| perm_not_spam | bool | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'post_set_comment_spam_status 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_set_spam_status() -> Result<(), Error> {
    let params: PostSetCommentSpamStatusParams = PostSetCommentSpamStatusParams {
        comment_id: String::from("acme-news/2026/06/19/article-84/comment-1023"),
        spam: Some(true),
        perm_not_spam: Some(false),
        sso: Some(String::from("jwt:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakepayload")),
    };
    let response: ApiEmptyResponse = post_set_comment_spam_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---