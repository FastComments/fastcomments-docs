## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| adjust_comment_votes_params | models::AdjustCommentVotesParams | Ναι |  |
| broadcast_id | String | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/adjust_votes_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'post_adjust_comment_votes Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---