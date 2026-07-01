## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| voteId | string | 是 |  |
| options | const DeleteModerationVoteOptions& | 是 |  |

## 响应

返回：[`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## 示例

[inline-code-attrs-start title = 'deleteModerationVote 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-abc123");
auto voteId = utility::conversions::to_string_t("vote-xyz789");
DeleteModerationVoteOptions opts;
opts.reason = utility::conversions::to_string_t("Offensive language");
opts.hardDelete = boost::optional<bool>(true);
api->deleteModerationVote(tenantId, commentId, voteId, opts)
    .then([](std::shared_ptr<VoteDeleteResponse> resp){});
[inline-code-end]