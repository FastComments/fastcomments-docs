## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| broadcast_id | String | Да |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример un_pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<ChangeCommentPinStatusResponse, Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-8f3b2a1e"),
        broadcast_id: String::from("news/2024/product-launch"),
        sso: Some(String::from("sso-user-abcdef123456")),
    };
    let response: ChangeCommentPinStatusResponse = un_pin_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---