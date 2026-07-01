## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenant_id | String | Yes |  |
| comment_id | String | No |  |
| external_id | String | No |  |
| event_type | String | No |  |
| domain | String | No |  |
| attempt_count_gt | f64 | No |  |

## Απόκριση

Επιστρέφει: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pending_webhook_event_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetPendingWebhookEventCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("comment-12345".to_string()),
        external_id: Some("ext-98765".to_string()),
        event_type: Some("comment_created".to_string()),
        domain: Some("news.example.com".to_string()),
        attempt_count_gt: Some(2.0),
    };
    let _response = get_pending_webhook_event_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]