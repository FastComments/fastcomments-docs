## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tag | String | 예 |  |
| tenant_id | String | 아니오 |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | 아니오 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_hash_tag 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteHashTagParams = DeleteHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    delete_hash_tag_request_body: Some(DeleteHashTagRequestBody {}),
};
let response: ApiEmptyResponse = delete_hash_tag(&configuration, params).await?;
[inline-code-end]

---