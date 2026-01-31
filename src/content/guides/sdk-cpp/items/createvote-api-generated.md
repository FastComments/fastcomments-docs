## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`VoteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteComment_200_response.h)

## Example

[inline-code-attrs-start title = 'createVote Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
int main()
{
    utility::string_t tenantId = U("my-tenant-123");
    utility::string_t commentId = U("cmt-456");
    utility::string_t direction = U("up");
    boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
    boost::optional<utility::string_t> anonUserId = boost::none;
    auto task = api->createVote(tenantId, commentId, direction, userId, anonUserId)
    .then([](std::shared_ptr<VoteComment_200_response> resp){
        auto copy = std::make_shared<VoteComment_200_response>(*resp);
        (void)copy;
    });
    task.wait();
    return 0;
}
[inline-code-end]
