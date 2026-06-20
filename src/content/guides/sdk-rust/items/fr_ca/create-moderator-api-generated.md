## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| create_moderator_body | models::CreateModeratorBody | Oui |  |

## Réponse

Renvoie : [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateModeratorParams = CreateModeratorParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_moderator_body: models::CreateModeratorBody {
        email: "jane.doe@acme-corp.com".to_string(),
        display_name: Some("Jane Doe".to_string()),
        username: Some("jdoe".to_string()),
        role: Some("moderator".to_string()),
        sections: Some(vec!["news/article".to_string(), "tech/reviews".to_string()]),
        notify: Some(true),
    },
};
let response: CreateModeratorResponse = create_moderator(&configuration, params).await?;
[inline-code-end]

---