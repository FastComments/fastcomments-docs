## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenant_id | String | Ja |  |
| post_id | String | Ja |  |
| update_feed_post_params | models::UpdateFeedPostParams | Ja |  |
| broadcast_id | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## Beispiel

[inline-code-attrs-start title = 'update_feed_post_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".into(),
    post_id: "news/article-123".into(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: Some("Updated Headline".into()),
        content: Some("Revised content of the article with latest information.".into()),
        ..Default::default()
    },
    broadcast_id: Some("broadcast-001".into()),
    sso: Some("sso-token-abc123".into()),
};

let response: CreateFeedPostResponse = update_feed_post_public(&configuration, params).await?;
[inline-code-end]