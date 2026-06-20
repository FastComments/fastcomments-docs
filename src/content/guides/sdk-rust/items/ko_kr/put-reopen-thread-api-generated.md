## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| url_id | String | 예 |  |
| sso | String | 아니요 |  |

## 반환

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'put_reopen_thread 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reopen_thread() -> Result<(), Error> {
    let params: PutReopenThreadParams = PutReopenThreadParams {
        url_id: String::from("acme-corp/news/article-2026-06-19"),
        sso: Some(String::from("sso-token-9f8e7d6c")),
    };
    let response: ApiEmptyResponse = put_reopen_thread(configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---