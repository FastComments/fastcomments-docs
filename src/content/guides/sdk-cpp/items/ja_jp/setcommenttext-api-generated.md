## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Yes |  |
| options | const SetCommentTextOptions& | Yes |  |

## 応答

戻り値: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPISetCommentTextResponse.h)

## 例

[inline-code-attrs-start title = 'setCommentText の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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