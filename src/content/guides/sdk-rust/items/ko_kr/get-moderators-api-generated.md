## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| skip | f64 | 아니요 |  |

## 응답

반환: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_moderators 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let moderators: GetModerators200Response = get_moderators(&configuration, params).await?;
    let _moderators = moderators;
    Ok(())
}
[inline-code-end]

---