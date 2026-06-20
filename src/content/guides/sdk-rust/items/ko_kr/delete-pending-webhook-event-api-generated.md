## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_pending_webhook_event 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn perform_delete() -> Result<ApiEmptyResponse, Error> {
    let tenant_id: Option<String> = Some(String::from("acme-corp-tenant"));
    let id: Option<String> = Some(String::from("wh_evt_2026_09f3"));
    let params: DeletePendingWebhookEventParams = DeletePendingWebhookEventParams {
        tenant_id: tenant_id.unwrap(),
        id: id.unwrap(),
    };
    let response: ApiEmptyResponse = delete_pending_webhook_event(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---