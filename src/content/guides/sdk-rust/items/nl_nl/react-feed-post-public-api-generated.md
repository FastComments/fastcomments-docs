## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| post_id | String | Ja |  |
| react_body_params | models::ReactBodyParams | Ja |  |
| is_undo | bool | Nee |  |
| broadcast_id | String | Nee |  |
| sso | String | Nee |  |

## Response

Retourneert: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'react_feed_post_public Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: ReactFeedPostPublicParams = ReactFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-06-19".to_string(),
        react_body_params: models::ReactBodyParams {
            reaction: "like".to_string(),
            user_id: "user-9876".to_string(),
            metadata: None,
        },
        is_undo: Some(false),
        broadcast_id: Some("broadcast-42".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: ReactFeedPostResponse = react_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---