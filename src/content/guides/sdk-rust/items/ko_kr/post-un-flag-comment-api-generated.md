## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'post_un_flag_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unflag_comment() -> Result<ApiEmptyResponse, Error> {
    let params: PostUnFlagCommentParams = PostUnFlagCommentParams {
        comment_id: "news/world/2026/06/19/comment-7890".to_string(),
        sso: Some("acme-corp-user-xyZ12Token".to_string()),
    };
    let response: ApiEmptyResponse = post_un_flag_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---