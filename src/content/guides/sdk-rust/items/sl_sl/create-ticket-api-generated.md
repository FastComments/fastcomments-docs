---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| user_id | String | Da |  |
| create_ticket_body | models::CreateTicketBody | Da |  |

## Odgovor

Vrača: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_ticket_200_response.rs)

## Primer

[inline-code-attrs-start title = 'create_ticket Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn create_ticket_example() -> Result<CreateTicket200Response, Error> {
    let params = CreateTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-12345".to_string(),
        create_ticket_body: models::CreateTicketBody {
            subject: "Moderation request: abusive comments".to_string(),
            description: "Several abusive comments reported on the article 'Election 2026: Key Races'".to_string(),
            priority: Some("high".to_string()),
            tags: Some(vec!["moderation".to_string(), "priority-high".to_string()]),
            reporter_email: Some("editor@news.example.com".to_string()),
            source_url: Some("https://news.example.com/articles/election-2026-key-races".to_string()),
            custom_fields: Some(std::collections::HashMap::from([
                ("article_id".to_string(), "news-20260324-001".to_string())
            ])),
        },
    };
    let response: CreateTicket200Response = create_ticket(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---