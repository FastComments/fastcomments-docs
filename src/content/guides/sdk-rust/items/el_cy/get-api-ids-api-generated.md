## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenant_id | String | Ναι |  |
| text_search | String | Όχι |  |
| by_ip_from_comment | String | Όχι |  |
| filters | String | Όχι |  |
| search_filters | String | Όχι |  |
| after_id | String | Όχι |  |
| demo | bool | Όχι |  |
| sso | String | Όχι |  |

## Απάντηση

Επιστρέφει: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'get_api_ids Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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