## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| url_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'put_reopen_thread 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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