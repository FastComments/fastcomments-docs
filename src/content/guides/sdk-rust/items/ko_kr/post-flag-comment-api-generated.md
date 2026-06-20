## 매개변수

| 이름 | Type | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'post_flag_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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