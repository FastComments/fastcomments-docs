---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| username_starts_with | String | Да |  |
| mention_group_ids | Vec<String> | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_200_response.rs)

---