## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_moderator_body | models::CreateModeratorBody | Yes |  |

## Svar

Returnerer: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'create_moderator Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateModeratorParams = CreateModeratorParams {
        tenant_id: String::from("acme-corp-tenant"),
        create_moderator_body: models::CreateModeratorBody {
            username: String::from("jane.moderator"),
            email: String::from("jane.moderator@acme.com"),
            display_name: Some(String::from("Jane Moderator")),
            sections: Some(vec![String::from("news/article"), String::from("forums/general")]),
            active: Some(true),
            notes: Some(String::from("Senior moderator, PST timezone")),
        },
    };
    let response: CreateModerator200Response = create_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---