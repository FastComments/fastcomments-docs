## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | f64 | No |  |

## Response

Returns: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_moderators Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_moderators() -> Result<GetModerators200Response, Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: String::from("acme-corp-tenant"),
        skip: Some(20.0),
    };
    let moderators: GetModerators200Response = get_moderators(&configuration, params).await?;
    Ok(moderators)
}
[inline-code-end]
