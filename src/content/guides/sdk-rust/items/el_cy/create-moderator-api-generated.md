## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| create_moderator_body | models::CreateModeratorBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'create_moderator Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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