## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Yes |  |
| options | const SetCommentTextOptions& | Yes |  |

## תגובה

מחזיר: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPISetCommentTextResponse.h)

## דוגמה

[inline-code-attrs-start title = 'setCommentText דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentId = U("cmt-456");
auto broadcastId = U("brd-789");

CommentTextUpdateRequest updateReq;
updateReq.text = U("Updated comment content");
updateReq.isEdited = boost::make_optional(true);

SetCommentTextOptions opts;
opts.notifySubscribers = boost::make_optional(true);

api->setCommentText(tenantId, commentId, broadcastId, updateReq, opts)
    .then([](std::shared_ptr<PublicAPISetCommentTextResponse> resp) {
    });
[inline-code-end]