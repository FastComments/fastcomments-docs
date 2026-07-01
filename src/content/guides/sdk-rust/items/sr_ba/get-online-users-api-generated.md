Тренутни онлајн гледаоци странице: људи чија вебсокет сесија је тренутно претплаћена на страницу.  
Враћа anonCount + totalCount (претплатници у целој соби, укључујући анонимне гледаоце које не набрајавамо).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| after_name | String | Не |  |
| after_user_id | String | Не |  |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Пример

[inline-code-attrs-start title = 'get_online_users Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]