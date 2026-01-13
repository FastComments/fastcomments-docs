## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | いいえ |  |
| external_id | String | いいえ |  |
| event_type | String | いいえ |  |
| domain | String | いいえ |  |
| attempt_count_gt | f64 | いいえ |  |

## レスポンス

戻り値: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_pending_webhook_event_count の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetPendingWebhookEventCount200Response, Error> {
    let params: GetPendingWebhookEventCountParams = GetPendingWebhookEventCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("cmt-12345".to_string()),
        external_id: Some("news/article-98765".to_string()),
        event_type: Some("comment.created".to_string()),
        domain: Some("news.example.com".to_string()),
        attempt_count_gt: Some(2.0),
    };
    let response: GetPendingWebhookEventCount200Response =
        get_pending_webhook_event_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---