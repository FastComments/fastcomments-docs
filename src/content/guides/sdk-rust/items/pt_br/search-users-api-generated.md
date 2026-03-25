## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| username_starts_with | String | Não |  |
| mention_group_ids | Vec<String> | Não |  |
| sso | String | Não |  |
| search_section | String | Não |  |

## Resposta

Retorna: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_search() -> Result<SearchUsers200Response, Error> {
    let params: SearchUsersParams = SearchUsersParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/article-2026-03-25-launch"),
        username_starts_with: Some(String::from("alex")),
        mention_group_ids: Some(vec![String::from("team-marketing"), String::from("team-product")]),
        sso: Some(String::from("okta")),
        search_section: Some(String::from("comments")),
    };
    let search_result: SearchUsers200Response = search_users(&configuration, params).await?;
    Ok(search_result)
}
[inline-code-end]

---