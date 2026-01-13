## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_moderator_body | models::CreateModeratorBody | Да |  |

## Одговор

Враћа: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_200_response.rs)

## Пример

[inline-code-attrs-start title = 'create_moderator Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_create_moderator() -> Result<(), Error> {
    let params: CreateModeratorParams = CreateModeratorParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        create_moderator_body: models::CreateModeratorBody {
            email: "jane.doe@acme.com".to_owned(),
            display_name: "Jane Doe".to_owned(),
            role: Some("moderator".to_owned()),
            active: Some(true),
            notes: Some("Handles product and support forums".to_owned()),
            permissions: Some(vec!["approve_comment".to_owned(), "delete_comment".to_owned()]),
        },
    };

    let _response: CreateModerator200Response = create_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---