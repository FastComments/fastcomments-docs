req
tenantId
afterId

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니요 |  |
| limit | i32 | 아니요 |  |
| tags | Vec<String> | 아니요 |  |
| sso | String | 아니요 |  |
| is_crawler | bool | 아니요 |  |
| include_user_info | bool | 아니요 |  |

## 응답

반환: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)