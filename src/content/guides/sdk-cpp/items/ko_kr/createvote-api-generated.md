## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| direction | string | 예 |  |
| options | const CreateVoteOptions& | 예 |  |

## 응답

반환: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## 예시

[inline-code-attrs-start title = 'createVote 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456");
auto direction = utility::conversions::to_string_t("up");
auto optionsPtr = std::make_shared<CreateVoteOptions>();
optionsPtr->userId = utility::conversions::to_string_t("user-789");
optionsPtr->ipAddress = boost::optional<utility::string_t>(utility::conversions::to_string_t("192.168.1.100"));
api->createVote(tenantId, commentId, direction, *optionsPtr)
    .then([](std::shared_ptr<VoteResponse> resp){});
[inline-code-end]

---