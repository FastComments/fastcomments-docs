## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| skip | f64 | 아니오 |  |

## 응답

반환: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_response.rs)

## 예제

[inline-code-attrs-start title = 'get_moderators 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_moderators(configuration: &configuration::Configuration) -> Result<GetModeratorsResponse, Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetModeratorsResponse = get_moderators(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---