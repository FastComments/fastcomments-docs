## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| reviewed | bool | Nee |  |
| sso | String | Nee |  |

## Response

Geeft terug: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'post_set_comment_review_status Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn set_comment_review_status() -> Result<ApiEmptyResponse, Error> {
    let params: PostSetCommentReviewStatusParams = PostSetCommentReviewStatusParams {
        comment_id: "news/article-2026-06-18-cmt-9843".to_string(),
        reviewed: Some(true),
        sso: Some("acme-sso-session-7f2e9b".to_string()),
    };
    let response: ApiEmptyResponse = post_set_comment_review_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---