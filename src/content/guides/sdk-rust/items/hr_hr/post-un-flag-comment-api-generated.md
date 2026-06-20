## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| comment_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'post_un_flag_comment Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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