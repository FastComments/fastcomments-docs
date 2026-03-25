## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 아니오 |  |
| external_id | String | 아니오 |  |
| event_type | String | 아니오 |  |
| domain | String | 아니오 |  |
| attempt_count_gt | f64 | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_pending_webhook_event_count 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_pending_webhook_count(configuration: &configuration::Configuration) -> Result<GetPendingWebhookEventCount200Response, Error> {
    let params: GetPendingWebhookEventCountParams = GetPendingWebhookEventCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: Some(String::from("cmt-12345")),
        external_id: Some(String::from("ext-67890")),
        event_type: Some(String::from("comment_posted")),
        domain: Some(String::from("news.example.com")),
        attempt_count_gt: Some(3.0),
    };
    let response: GetPendingWebhookEventCount200Response = get_pending_webhook_event_count(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---