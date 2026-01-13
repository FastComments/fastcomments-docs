## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPagesAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'getPages 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
int main()
{
    utility::string_t tenantId = U("my-tenant-123");
    boost::optional<utility::string_t> statusFilter = U("published");
    boost::optional<int> pageNumber = 1;
    auto placeholder = std::make_shared<GetPagesAPIResponse>();
    api->getPages(tenantId)
       .then([placeholder](std::shared_ptr<GetPagesAPIResponse> resp) {
           if (resp) {
               *placeholder = *resp;
               auto refs = resp.use_count();
               (void)refs;
           }
       })
       .wait();
    return 0;
}
[inline-code-end]

---