## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 아니오 |  |
| external_id | String | 아니오 |  |
| event_type | String | 아니오 |  |
| domain | String | 아니오 |  |
| attempt_count_gt | f64 | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_response.rs)

## 예제

[inline-code-attrs-start title = 'get_pending_webhook_event_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetPendingWebhookEventCountParams = GetPendingWebhookEventCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("cmt_92a7b3".to_string()),
        external_id: Some("article-2026-06-19".to_string()),
        event_type: Some("comment.created".to_string()),
        domain: Some("acme.com".to_string()),
        attempt_count_gt: Some(1.0),
    };
    let count_response: GetPendingWebhookEventCountResponse = get_pending_webhook_event_count(cfg, params).await?;
    Ok(())
}
[inline-code-end]

---