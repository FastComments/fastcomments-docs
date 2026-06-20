## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| comment_id | String | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di post_un_flag_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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