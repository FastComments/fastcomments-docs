## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| text_search | String | Nee |  |
| by_ip_from_comment | String | Nee |  |
| filters | String | Nee |  |
| search_filters | String | Nee |  |
| after_id | String | Nee |  |
| demo | bool | Nee |  |
| sso | String | Nee |  |

## Response

Retourneert: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_api_ids Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetApiIdsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: None,
        after_id: None,
        demo: Some(false),
        sso: Some("sso-token".to_string()),
    };
    let _response = get_api_ids(&config, params).await?;
    Ok(())
}
[inline-code-end]