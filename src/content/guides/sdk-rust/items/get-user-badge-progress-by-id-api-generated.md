## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge_progress_by_id Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_user_badge_progress() -> Result<(), Error> {
    let params: GetUserBadgeProgressByIdParams = GetUserBadgeProgressByIdParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("badge-92b7"),
    };
    let correlation_id: Option<String> = Some(String::from("req-20260113-0001"));
    let response: GetUserBadgeProgressById200Response =
        get_user_badge_progress_by_id(&configuration, params).await?;
    let _progress: UserBadgeProgress = response.into();
    Ok(())
}
[inline-code-end]
