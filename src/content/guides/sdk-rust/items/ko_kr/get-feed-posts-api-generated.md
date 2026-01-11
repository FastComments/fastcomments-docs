req
tenantId
afterId

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니오 |  |
| limit | i32 | 아니오 |  |
| tags | Vec<String> | 아니오 |  |

## 응답

반환: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)