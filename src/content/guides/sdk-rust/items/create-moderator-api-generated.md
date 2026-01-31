## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_moderator_body | models::CreateModeratorBody | Yes |  |

## Response

Returns: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_moderator Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<CreateModerator200Response, Error> {
    let params: CreateModeratorParams = CreateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_moderator_body: models::CreateModeratorBody {
            username: "jane.doe".to_string(),
            email: "jane.doe@acme-corp.com".to_string(),
            display_name: Some("Jane Doe".to_string()),
            roles: Some(vec!["moderator".to_string(), "senior_moderator".to_string()]),
            notify: Some(true),
        },
    };
    let response: CreateModerator200Response = create_moderator(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
