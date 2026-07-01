## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| create_feed_post_params | models::CreateFeedPostParams | Ναι |  |
| broadcast_id | String | Όχι |  |
| is_live | bool | Όχι |  |
| do_spam_check | bool | Όχι |  |
| skip_dup_check | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'create_feed_post Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".into(),
        create_feed_post_params: models::CreateFeedPostParams {
            text: "Launching new features".into(),
            media: vec![],
        },
        broadcast_id: Some("broadcast-2023-09".into()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let _response = create_feed_post(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---