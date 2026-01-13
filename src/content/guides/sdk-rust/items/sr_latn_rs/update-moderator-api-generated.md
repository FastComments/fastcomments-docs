## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_moderator_body | models::UpdateModeratorBody | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Primer

[inline-code-attrs-start title = 'update_moderator Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateModeratorParams = UpdateModeratorParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "moderator-12345".to_string(),
    update_moderator_body: models::UpdateModeratorBody {
        username: "jane.doe".to_string(),
        display_name: Some("Jane Doe".to_string()),
        email: Some("jane.doe@acme.com".to_string()),
        is_active: Some(true),
        permissions: Some(vec!["moderate_comments".to_string(), "view_reports".to_string()]),
    },
};
let response: FlagCommentPublic200Response = update_moderator(&configuration, params).await?;
[inline-code-end]

---