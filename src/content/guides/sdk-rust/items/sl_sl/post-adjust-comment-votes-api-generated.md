## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| adjust_comment_votes_params | models::AdjustCommentVotesParams | Da |  |
| broadcast_id | String | Ne |  |
| ssoa | String | Ne |  |

## Odgovor

Vrne: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/adjust_votes_response.rs)

## Primer

[inline-code-attrs-start title = 'post_adjust_comment_votes Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn adjust_votes_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostAdjustCommentVotesParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "comment-9876".to_string(),
        adjust_comment_votes_params: models::AdjustCommentVotesParams::default(),
        broadcast_id: Some("broadcast-2023-11".to_string()),
        sso: Some("sso-xyz".to_string()),
    };
    let _response = post_adjust_comment_votes(configuration, params).await?;
    Ok(())
}
[inline-code-end]