## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Yes |  |

## Response

Returns: [`CreateQuestionConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'createQuestionConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateQuestionConfigBody>();
bodyPtr->name = U("Product A Feedback");
bodyPtr->description = U("Configuration for Product A feedback questions");
bodyPtr->createdBy = U("admin@example.com");
bodyPtr->notifyEmail = boost::optional<utility::string_t>(U("notify@example.com"));
bodyPtr->isRequired = true;
api->createQuestionConfig(tenantId, *bodyPtr)
    .then([](std::shared_ptr<CreateQuestionConfig_200_response> resp){
        if (resp) std::cout << "Question config created successfully\n";
        else std::cout << "Failed to create question config\n";
    })
    .wait();
[inline-code-end]
