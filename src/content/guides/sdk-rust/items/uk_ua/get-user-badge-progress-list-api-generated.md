## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| user_id | String | Ні |  |
| limit | f64 | Ні |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_list_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_badge_progress_list'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgeProgressListParams = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user_98765".to_string()),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let response: GetUserBadgeProgressList200Response =
        get_user_badge_progress_list(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---