## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 아니오 |  |
| external_id | String | 아니오 |  |
| event_type | String | 아니오 |  |
| domain | String | 아니오 |  |
| attempt_count_gt | f64 | 아니오 |  |
| skip | f64 | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_pending_webhook_events 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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