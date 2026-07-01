## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`DeletePageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_page_api_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_page 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeletePageParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "news/article".into(),
    };
    let _resp = delete_page(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---