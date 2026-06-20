## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| page | f64 | Nej |  |
| count | f64 | Nej |  |
| text_search | String | Nej |  |
| by_ip_from_comment | String | Nej |  |
| filters | String | Nej |  |
| search_filters | String | Nej |  |
| sorts | String | Nej |  |
| demo | bool | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_api_comments Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetApiCommentsParams = GetApiCommentsParams {
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some(String::from("breaking election results")),
        by_ip_from_comment: Some(String::from("203.0.113.45")),
        filters: Some(String::from("status:approved,thread:news/article")),
        search_filters: Some(String::from("author:john.doe@example.com")),
        sorts: Some(String::from("-created_at")),
        demo: Some(false),
        sso: Some(String::from("acme-corp-tenant")),
    };
    let response: ModerationApiGetCommentsResponse = get_api_comments(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---