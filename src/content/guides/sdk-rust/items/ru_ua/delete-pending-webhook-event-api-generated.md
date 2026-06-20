## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Ответ

Возвращает: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример delete_pending_webhook_event'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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