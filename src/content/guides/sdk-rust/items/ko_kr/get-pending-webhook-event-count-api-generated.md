## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| comment_id | String | 아니오 |  |
| external_id | String | 아니오 |  |
| event_type | String | 아니오 |  |
| domain | String | 아니오 |  |
| attempt_count_gt | f64 | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_response.rs)

## 예시

[inline-code-attrs-start title = 'get_pending_webhook_event_count 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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