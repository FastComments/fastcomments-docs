## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 否 |  |
| external_id | String | 否 |  |
| event_type | String | 否 |  |
| domain | String | 否 |  |
| attempt_count_gt | f64 | 否 |  |
| skip | f64 | 否 |  |

## 回應

回傳: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_pending_webhook_events 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPendingWebhookEventsParams = GetPendingWebhookEventsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: Some("cmt-12345".to_string()),
    external_id: Some("article-98765".to_string()),
    event_type: Some("comment.create".to_string()),
    domain: Some("news.example.com".to_string()),
    attempt_count_gt: Some(1.0),
    skip: Some(0.0),
};

let pending: GetPendingWebhookEvents200Response = get_pending_webhook_events(&configuration, params).await?;
[inline-code-end]

---