## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| username_starts_with | String | Nie |  |
| mention_group_ids | Vec<String> | Nie |  |
| sso | String | Nie |  |
| search_section | String | Nie |  |

## Odpowiedź

Zwraca: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_users() -> Result<(), Error> {
    let params: SearchUsersParams = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-2026-06".to_string(),
        username_starts_with: Some("jo".to_string()),
        mention_group_ids: Some(vec![
            "group-moderators".to_string(),
            "group-editors".to_string(),
        ]),
        sso: Some("google".to_string()),
        search_section: Some("comments".to_string()),
    };

    let result: SearchUsersResult = search_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---