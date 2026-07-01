## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| post_id | String | Ναι |  |
| react_body_params | models::ReactBodyParams | Ναι |  |
| is_undo | bool | Όχι |  |
| broadcast_id | String | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'react_feed_post_public Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let react_body = models::ReactBodyParams {
        reaction: "like".to_string(),
    };
    let params = ReactFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article/12345".to_string(),
        react_body_params: react_body,
        is_undo: Some(false),
        broadcast_id: Some("broadcast-xyz".to_string()),
        sso: Some("sso-token-abc".to_string()),
    };
    let _response: ReactFeedPostResponse = react_feed_post_public(&config, params).await?;
    Ok(())
}
[inline-code-end]