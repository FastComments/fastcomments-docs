# fastcomments のドキュメント

<a name="documentation-for-api-endpoints"></a>
## API エンドポイントのドキュメント

All URIs are relative to *https://fastcomments.com*

| クラス | メソッド | HTTP リクエスト | 説明 |
|------------ | ------------- | ------------- | -------------|
| *DefaultApi* | [**addDomainConfig**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#addDomainConfig) | **POST** /api/v1/domain-configs |  |
*DefaultApi* | [**addPage**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#addPage) | **POST** /api/v1/pages |  |
*DefaultApi* | [**addSSOUser**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#addSSOUser) | **POST** /api/v1/sso-users |  |
*DefaultApi* | [**aggregate**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#aggregate) | **POST** /api/v1/aggregate | ドキュメントをグループ化して（groupBy が指定されている場合）複数の操作を適用して集計します。sum、countDistinct、avg など、さまざまな操作がサポートされています。 |
*DefaultApi* | [**aggregateQuestionResults**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#aggregateQuestionResults) | **GET** /api/v1/question-results-aggregation |  |
*DefaultApi* | [**blockUserFromComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#blockUserFromComment) | **POST** /api/v1/comments/{id}/block |  |
*DefaultApi* | [**bulkAggregateQuestionResults**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#bulkAggregateQuestionResults) | **POST** /api/v1/question-results-aggregation/bulk |  |
*DefaultApi* | [**combineCommentsWithQuestionResults**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#combineCommentsWithQuestionResults) | **GET** /api/v1/question-results-aggregation/combine/comments |  |
*DefaultApi* | [**createFeedPost**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#createFeedPost) | **POST** /api/v1/feed-posts |  |
*DefaultApi* | [**createSubscription**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#createSubscription) | **POST** /api/v1/subscriptions |  |
*DefaultApi* | [**createUserBadge**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#createUserBadge) | **POST** /api/v1/user-badges |  |
*DefaultApi* | [**deleteComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#deleteComment) | **DELETE** /api/v1/comments/{id} |  |
*DefaultApi* | [**deleteDomainConfig**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#deleteDomainConfig) | **DELETE** /api/v1/domain-configs/{domain} |  |
*DefaultApi* | [**deletePage**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#deletePage) | **DELETE** /api/v1/pages/{id} |  |
*DefaultApi* | [**deleteSSOUser**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#deleteSSOUser) | **DELETE** /api/v1/sso-users/{id} |  |
*DefaultApi* | [**deleteSubscription**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#deleteSubscription) | **DELETE** /api/v1/subscriptions/{id} |  |
*DefaultApi* | [**deleteUserBadge**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#deleteUserBadge) | **DELETE** /api/v1/user-badges/{id} |  |
*DefaultApi* | [**flagComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#flagComment) | **POST** /api/v1/comments/{id}/flag |  |
*DefaultApi* | [**getAuditLogs**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getAuditLogs) | **GET** /api/v1/audit-logs |  |
*DefaultApi* | [**getComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getComment) | **GET** /api/v1/comments/{id} |  |
*DefaultApi* | [**getComments**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getComments) | **GET** /api/v1/comments |  |
*DefaultApi* | [**getDomainConfig**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getDomainConfig) | **GET** /api/v1/domain-configs/{domain} |  |
*DefaultApi* | [**getDomainConfigs**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getDomainConfigs) | **GET** /api/v1/domain-configs |  |
*DefaultApi* | [**getFeedPosts**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getFeedPosts) | **GET** /api/v1/feed-posts | 必須: tenantId afterId |
*DefaultApi* | [**getPageByURLId**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getPageByURLId) | **GET** /api/v1/pages/by-url-id |  |
*DefaultApi* | [**getPages**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getPages) | **GET** /api/v1/pages |  |
*DefaultApi* | [**getSSOUserByEmail**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getSSOUserByEmail) | **GET** /api/v1/sso-users/by-email/{email} |  |
*DefaultApi* | [**getSSOUserById**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getSSOUserById) | **GET** /api/v1/sso-users/by-id/{id} |  |
*DefaultApi* | [**getSSOUsers**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getSSOUsers) | **GET** /api/v1/sso-users |  |
*DefaultApi* | [**getSubscriptions**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getSubscriptions) | **GET** /api/v1/subscriptions |  |
*DefaultApi* | [**getUserBadge**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getUserBadge) | **GET** /api/v1/user-badges/{id} |  |
*DefaultApi* | [**getUserBadgeProgressById**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getUserBadgeProgressById) | **GET** /api/v1/user-badge-progress/{id} |  |
*DefaultApi* | [**getUserBadgeProgressByUserId**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getUserBadgeProgressByUserId) | **GET** /api/v1/user-badge-progress/user/{userId} |  |
*DefaultApi* | [**getUserBadgeProgressList**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getUserBadgeProgressList) | **GET** /api/v1/user-badge-progress |  |
*DefaultApi* | [**getUserBadges**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#getUserBadges) | **GET** /api/v1/user-badges |  |
*DefaultApi* | [**patchDomainConfig**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#patchDomainConfig) | **PATCH** /api/v1/domain-configs/{domainToUpdate} |  |
*DefaultApi* | [**patchPage**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#patchPage) | **PATCH** /api/v1/pages/{id} |  |
*DefaultApi* | [**patchSSOUser**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#patchSSOUser) | **PATCH** /api/v1/sso-users/{id} |  |
*DefaultApi* | [**putDomainConfig**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#putDomainConfig) | **PUT** /api/v1/domain-configs/{domainToUpdate} |  |
*DefaultApi* | [**putSSOUser**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#putSSOUser) | **PUT** /api/v1/sso-users/{id} |  |
*DefaultApi* | [**saveComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#saveComment) | **POST** /api/v1/comments |  |
*DefaultApi* | [**saveCommentsBulk**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#saveCommentsBulk) | **POST** /api/v1/comments/bulk |  |
*DefaultApi* | [**unBlockUserFromComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#unBlockUserFromComment) | **POST** /api/v1/comments/{id}/un-block |  |
*DefaultApi* | [**unFlagComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#unFlagComment) | **POST** /api/v1/comments/{id}/un-flag |  |
*DefaultApi* | [**updateComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#updateComment) | **PATCH** /api/v1/comments/{id} |  |
*DefaultApi* | [**updateFeedPost**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#updateFeedPost) | **PATCH** /api/v1/feed-posts/{id} |  |
*DefaultApi* | [**updateUserBadge**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/DefaultApi.md#updateUserBadge) | **PUT** /api/v1/user-badges/{id} |  |
| *PublicApi* | [**blockFromCommentPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#blockFromCommentPublic) | **POST** /block-from-comment/{commentId} |  |
*PublicApi* | [**checkedCommentsForBlocked**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#checkedCommentsForBlocked) | **GET** /check-blocked-comments |  |
*PublicApi* | [**createCommentPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#createCommentPublic) | **POST** /comments/{tenantId} |  |
*PublicApi* | [**createFeedPostPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#createFeedPostPublic) | **POST** /feed-posts/{tenantId} |  |
*PublicApi* | [**deleteCommentPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#deleteCommentPublic) | **DELETE** /comments/{tenantId}/{commentId} |  |
*PublicApi* | [**deleteCommentVote**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#deleteCommentVote) | **DELETE** /comments/{tenantId}/{commentId}/vote/{voteId} |  |
*PublicApi* | [**deleteFeedPostPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#deleteFeedPostPublic) | **DELETE** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | [**flagCommentPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#flagCommentPublic) | **POST** /flag-comment/{commentId} |  |
*PublicApi* | [**getCommentText**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getCommentText) | **GET** /comments/{tenantId}/{commentId}/text |  |
*PublicApi* | [**getCommentVoteUserNames**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getCommentVoteUserNames) | **GET** /comments/{tenantId}/{commentId}/votes |  |
*PublicApi* | [**getCommentsPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getCommentsPublic) | **GET** /comments/{tenantId} | 必須: tenantId urlId |
*PublicApi* | [**getEventLog**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getEventLog) | **GET** /event-log/{tenantId} | 必須: tenantId urlId userIdWS |
*PublicApi* | [**getFeedPostsPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getFeedPostsPublic) | **GET** /feed-posts/{tenantId} | 必須: tenantId afterId |
*PublicApi* | [**getFeedPostsStats**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getFeedPostsStats) | **GET** /feed-posts/{tenantId}/stats |  |
*PublicApi* | [**getGlobalEventLog**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getGlobalEventLog) | **GET** /event-log/global/{tenantId} | 必須: tenantId urlId userIdWS |
*PublicApi* | [**getUserNotificationCount**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getUserNotificationCount) | **GET** /user-notifications/get-count |  |
*PublicApi* | [**getUserNotifications**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getUserNotifications) | **GET** /user-notifications |  |
*PublicApi* | [**getUserPresenceStatuses**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getUserPresenceStatuses) | **GET** /user-presence-status |  |
*PublicApi* | [**getUserReactsPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#getUserReactsPublic) | **GET** /feed-posts/{tenantId}/user-reacts |  |
*PublicApi* | [**lockComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#lockComment) | **POST** /comments/{tenantId}/{commentId}/lock |  |
*PublicApi* | [**pinComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#pinComment) | **POST** /comments/{tenantId}/{commentId}/pin |  |
*PublicApi* | [**reactFeedPostPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#reactFeedPostPublic) | **POST** /feed-posts/{tenantId}/react/{postId} |  |
*PublicApi* | [**resetUserNotificationCount**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#resetUserNotificationCount) | **POST** /user-notifications/reset-count |  |
*PublicApi* | [**resetUserNotifications**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#resetUserNotifications) | **POST** /user-notifications/reset |  |
*PublicApi* | [**searchUsers**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#searchUsers) | **GET** /user-search/{tenantId} |  |
*PublicApi* | [**setCommentText**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#setCommentText) | **POST** /comments/{tenantId}/{commentId}/update-text |  |
*PublicApi* | [**unBlockCommentPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#unBlockCommentPublic) | **DELETE** /block-from-comment/{commentId} |  |
*PublicApi* | [**unLockComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#unLockComment) | **POST** /comments/{tenantId}/{commentId}/unlock |  |
*PublicApi* | [**unPinComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#unPinComment) | **POST** /comments/{tenantId}/{commentId}/unpin |  |
*PublicApi* | [**updateFeedPostPublic**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#updateFeedPostPublic) | **PUT** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | [**updateUserNotificationCommentSubscriptionStatus**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#updateUserNotificationCommentSubscriptionStatus) | **POST** /user-notifications/{notificationId}/mark-opted/{optedInOrOut} | 特定のコメントに対する通知を有効または無効にします。 |
*PublicApi* | [**updateUserNotificationPageSubscriptionStatus**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#updateUserNotificationPageSubscriptionStatus) | **POST** /user-notifications/set-subscription-state/{subscribedOrUnsubscribed} | ページの通知を有効または無効にします。ユーザーがページに登録されていると、ルートコメントの新規投稿に対して通知が作成され、また |
*PublicApi* | [**updateUserNotificationStatus**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#updateUserNotificationStatus) | **POST** /user-notifications/{notificationId}/mark/{newStatus} |  |
*PublicApi* | [**uploadImage**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#uploadImage) | **POST** /upload-image/{tenantId} | 画像をアップロードしてリサイズします |
*PublicApi* | [**voteComment**](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Apis/PublicApi.md#voteComment) | **POST** /comments/{tenantId}/{commentId}/vote |  |


<a name="documentation-for-models"></a>
## モデルのドキュメント

 - [APIAuditLog](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIAuditLog.md)
 - [APIComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIComment.md)
 - [APICommentBase](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APICommentBase.md)
 - [APICreateUserBadgeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APICreateUserBadgeResponse.md)
 - [APIEmptyResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIEmptyResponse.md)
 - [APIEmptySuccessResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIEmptySuccessResponse.md)
 - [APIError](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIError.md)
 - [APIGetCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetCommentResponse.md)
 - [APIGetCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetCommentsResponse.md)
 - [APIGetUserBadgeProgressListResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgeProgressListResponse.md)
 - [APIGetUserBadgeProgressResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgeProgressResponse.md)
 - [APIGetUserBadgeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgeResponse.md)
 - [APIGetUserBadgesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgesResponse.md)
 - [APIPage](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIPage.md)
 - [APISSOUser](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APISSOUser.md)
 - [APIStatus](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIStatus.md)
 - [APIUserSubscription](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIUserSubscription.md)
 - [AddDomainConfigParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddDomainConfigParams.md)
 - [AddDomainConfig_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddDomainConfig_200_response.md)
 - [AddDomainConfig_200_response_anyOf](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddDomainConfig_200_response_anyOf.md)
 - [AddPageAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddPageAPIResponse.md)
 - [AddSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddSSOUserAPIResponse.md)
 - [AggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregateQuestionResultsResponse.md)
 - [AggregateQuestionResults_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregateQuestionResults_200_response.md)
 - [AggregateTimeBucket](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregateTimeBucket.md)
 - [AggregationItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationItem.md)
 - [AggregationOpType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationOpType.md)
 - [AggregationOperation](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationOperation.md)
 - [AggregationRequest](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationRequest.md)
 - [AggregationRequest_sort](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationRequest_sort.md)
 - [AggregationResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationResponse.md)
 - [AggregationResponse_stats](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationResponse_stats.md)
 - [AggregationValue](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationValue.md)
 - [BlockFromCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BlockFromCommentParams.md)
 - [BlockFromCommentPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BlockFromCommentPublic_200_response.md)
 - [BlockSuccess](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BlockSuccess.md)
 - [BulkAggregateQuestionItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionItem.md)
 - [BulkAggregateQuestionResultsRequest](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionResultsRequest.md)
 - [BulkAggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionResultsResponse.md)
 - [BulkAggregateQuestionResults_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionResults_200_response.md)
 - [ChangeCommentPinStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ChangeCommentPinStatusResponse.md)
 - [CheckBlockedCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CheckBlockedCommentsResponse.md)
 - [CheckedCommentsForBlocked_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CheckedCommentsForBlocked_200_response.md)
 - [CombineCommentsWithQuestionResults_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CombineCommentsWithQuestionResults_200_response.md)
 - [CombineQuestionResultsWithCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CombineQuestionResultsWithCommentsResponse.md)
 - [CommentData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentData.md)
 - [CommentHTMLRenderingMode](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentHTMLRenderingMode.md)
 - [CommentLogData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentLogData.md)
 - [CommentLogEntry](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentLogEntry.md)
 - [CommentLogType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentLogType.md)
 - [CommentQuestionResultsRenderingType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentQuestionResultsRenderingType.md)
 - [CommentQuestionsRequired](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentQuestionsRequired.md)
 - [CommentTextUpdateRequest](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentTextUpdateRequest.md)
 - [CommentThreadDeletionMode](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentThreadDeletionMode.md)
 - [CommentUserBadgeInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentUserBadgeInfo.md)
 - [CommentUserHashTagInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentUserHashTagInfo.md)
 - [CommentUserMentionInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentUserMentionInfo.md)
 - [CommenterNameFormats](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommenterNameFormats.md)
 - [CreateAPIPageData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateAPIPageData.md)
 - [CreateAPISSOUserData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateAPISSOUserData.md)
 - [CreateAPIUserSubscriptionData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateAPIUserSubscriptionData.md)
 - [CreateCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateCommentParams.md)
 - [CreateCommentPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateCommentPublic_200_response.md)
 - [CreateFeedPostParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostParams.md)
 - [CreateFeedPostPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostPublic_200_response.md)
 - [CreateFeedPostResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostResponse.md)
 - [CreateFeedPost_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPost_200_response.md)
 - [CreateFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostsResponse.md)
 - [CreateSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateSubscriptionAPIResponse.md)
 - [CreateUserBadgeParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateUserBadgeParams.md)
 - [CreateUserBadge_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateUserBadge_200_response.md)
 - [CustomConfigParameters](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CustomConfigParameters.md)
 - [DeleteCommentAction](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteCommentAction.md)
 - [DeleteCommentPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteCommentPublic_200_response.md)
 - [DeleteCommentResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteCommentResult.md)
 - [DeleteCommentVote_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteCommentVote_200_response.md)
 - [DeleteComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteComment_200_response.md)
 - [DeleteDomainConfig_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteDomainConfig_200_response.md)
 - [DeleteFeedPostPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteFeedPostPublic_200_response.md)
 - [DeleteFeedPostPublic_200_response_anyOf](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteFeedPostPublic_200_response_anyOf.md)
 - [DeletePageAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeletePageAPIResponse.md)
 - [DeleteSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteSSOUserAPIResponse.md)
 - [DeleteSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteSubscriptionAPIResponse.md)
 - [DeletedCommentResultComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeletedCommentResultComment.md)
 - [EventLogEntry](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/EventLogEntry.md)
 - [FComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FComment.md)
 - [FComment_meta](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FComment_meta.md)
 - [FeedPost](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FeedPost.md)
 - [FeedPostLink](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FeedPostLink.md)
 - [FeedPostMediaItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FeedPostMediaItem.md)
 - [FeedPostMediaItemAsset](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FeedPostMediaItemAsset.md)
 - [FeedPostStats](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FeedPostStats.md)
 - [FeedPostsStatsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FeedPostsStatsResponse.md)
 - [FindCommentsByRangeItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FindCommentsByRangeItem.md)
 - [FindCommentsByRangeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FindCommentsByRangeResponse.md)
 - [FlagCommentPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FlagCommentPublic_200_response.md)
 - [FlagCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FlagCommentResponse.md)
 - [FlagComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FlagComment_200_response.md)
 - [GetAuditLogsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetAuditLogsResponse.md)
 - [GetAuditLogs_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetAuditLogs_200_response.md)
 - [GetCommentText_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentText_200_response.md)
 - [GetCommentVoteUserNamesSuccessResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentVoteUserNamesSuccessResponse.md)
 - [GetCommentVoteUserNames_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentVoteUserNames_200_response.md)
 - [GetComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetComment_200_response.md)
 - [GetCommentsPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentsPublic_200_response.md)
 - [GetCommentsResponseWithPresence_PublicComment_](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentsResponseWithPresence_PublicComment_.md)
 - [GetCommentsResponse_PublicComment_](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentsResponse_PublicComment_.md)
 - [GetComments_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetComments_200_response.md)
 - [GetDomainConfig_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfig_200_response.md)
 - [GetDomainConfigs_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigs_200_response.md)
 - [GetDomainConfigs_200_response_anyOf](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigs_200_response_anyOf.md)
 - [GetDomainConfigs_200_response_anyOf_1](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigs_200_response_anyOf_1.md)
 - [GetEventLogResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEventLogResponse.md)
 - [GetEventLog_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEventLog_200_response.md)
 - [GetFeedPostsPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetFeedPostsPublic_200_response.md)
 - [GetFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetFeedPostsResponse.md)
 - [GetFeedPostsStats_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetFeedPostsStats_200_response.md)
 - [GetFeedPosts_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetFeedPosts_200_response.md)
 - [GetMyNotificationsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetMyNotificationsResponse.md)
 - [GetPageByURLIdAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPageByURLIdAPIResponse.md)
 - [GetPagesAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPagesAPIResponse.md)
 - [GetPublicFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPublicFeedPostsResponse.md)
 - [GetSSOUserByEmailAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSSOUserByEmailAPIResponse.md)
 - [GetSSOUserByIdAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSSOUserByIdAPIResponse.md)
 - [GetSSOUsers_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSSOUsers_200_response.md)
 - [GetSubscriptionsAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSubscriptionsAPIResponse.md)
 - [GetUserBadgeProgressById_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserBadgeProgressById_200_response.md)
 - [GetUserBadgeProgressList_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserBadgeProgressList_200_response.md)
 - [GetUserBadge_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserBadge_200_response.md)
 - [GetUserBadges_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserBadges_200_response.md)
 - [GetUserNotificationCountResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserNotificationCountResponse.md)
 - [GetUserNotificationCount_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserNotificationCount_200_response.md)
 - [GetUserNotifications_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserNotifications_200_response.md)
 - [GetUserPresenceStatusesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserPresenceStatusesResponse.md)
 - [GetUserPresenceStatuses_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserPresenceStatuses_200_response.md)
 - [GetUserReactsPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserReactsPublic_200_response.md)
 - [GifRating](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GifRating.md)
 - [HeaderState](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/HeaderState.md)
 - [IgnoredResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/IgnoredResponse.md)
 - [ImageContentProfanityLevel](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ImageContentProfanityLevel.md)
 - [LiveEvent](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LiveEvent.md)
 - [LiveEventType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LiveEventType.md)
 - [LiveEvent_extraInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LiveEvent_extraInfo.md)
 - [LockComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LockComment_200_response.md)
 - [MediaAsset](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/MediaAsset.md)
 - [MetaItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/MetaItem.md)
 - [NotificationAndCount](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/NotificationAndCount.md)
 - [NotificationObjectType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/NotificationObjectType.md)
 - [NotificationType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/NotificationType.md)
 - [PatchDomainConfigParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchDomainConfigParams.md)
 - [PatchPageAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchPageAPIResponse.md)
 - [PatchSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchSSOUserAPIResponse.md)
 - [PinComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PinComment_200_response.md)
 - [PubSubComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PubSubComment.md)
 - [PubSubCommentBase](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PubSubCommentBase.md)
 - [PubSubVote](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PubSubVote.md)
 - [PublicAPIDeleteCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicAPIDeleteCommentResponse.md)
 - [PublicAPIGetCommentTextResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicAPIGetCommentTextResponse.md)
 - [PublicAPISetCommentTextResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicAPISetCommentTextResponse.md)
 - [PublicBlockFromCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicBlockFromCommentParams.md)
 - [PublicComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicComment.md)
 - [PublicCommentBase](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicCommentBase.md)
 - [PublicFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicFeedPostsResponse.md)
 - [PutSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PutSSOUserAPIResponse.md)
 - [QueryPredicate](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QueryPredicate.md)
 - [QueryPredicate_value](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QueryPredicate_value.md)
 - [QuestionDatum](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionDatum.md)
 - [QuestionRenderingType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionRenderingType.md)
 - [QuestionResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionResult.md)
 - [QuestionResultAggregationOverall](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionResultAggregationOverall.md)
 - [QuestionSubQuestionVisibility](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionSubQuestionVisibility.md)
 - [QuestionWhenSave](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionWhenSave.md)
 - [ReactBodyParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReactBodyParams.md)
 - [ReactFeedPostPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReactFeedPostPublic_200_response.md)
 - [ReactFeedPostResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReactFeedPostResponse.md)
 - [Record_string__before_string_or_null__after_string_or_null___value](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/Record_string__before_string_or_null__after_string_or_null___value.md)
 - [Record_string_string_or_number__value](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/Record_string_string_or_number__value.md)
 - [RenderableUserNotification](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RenderableUserNotification.md)
 - [RepeatCommentCheckIgnoredReason](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RepeatCommentCheckIgnoredReason.md)
 - [RepeatCommentHandlingAction](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RepeatCommentHandlingAction.md)
 - [ResetUserNotificationsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ResetUserNotificationsResponse.md)
 - [ResetUserNotifications_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ResetUserNotifications_200_response.md)
 - [SORT_DIR](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SORT_DIR.md)
 - [SSOSecurityLevel](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SSOSecurityLevel.md)
 - [SaveCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveCommentResponse.md)
 - [SaveCommentResponseOptimized](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveCommentResponseOptimized.md)
 - [SaveComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveComment_200_response.md)
 - [SaveCommentsResponseWithPresence](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveCommentsResponseWithPresence.md)
 - [SearchUsersResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SearchUsersResponse.md)
 - [SearchUsers_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SearchUsers_200_response.md)
 - [SetCommentTextResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetCommentTextResult.md)
 - [SetCommentText_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetCommentText_200_response.md)
 - [SizePreset](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SizePreset.md)
 - [SortDirections](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SortDirections.md)
 - [SpamRule](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SpamRule.md)
 - [UnBlockCommentPublic_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UnBlockCommentPublic_200_response.md)
 - [UnBlockFromCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UnBlockFromCommentParams.md)
 - [UnblockSuccess](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UnblockSuccess.md)
 - [UpdatableCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdatableCommentParams.md)
 - [UpdateAPIPageData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateAPIPageData.md)
 - [UpdateAPISSOUserData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateAPISSOUserData.md)
 - [UpdateDomainConfigParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateDomainConfigParams.md)
 - [UpdateFeedPostParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateFeedPostParams.md)
 - [UpdateUserBadgeParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserBadgeParams.md)
 - [UpdateUserBadge_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserBadge_200_response.md)
 - [UpdateUserNotificationStatus_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserNotificationStatus_200_response.md)
 - [UploadImageResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UploadImageResponse.md)
 - [UserBadge](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserBadge.md)
 - [UserBadgeProgress](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserBadgeProgress.md)
 - [UserNotification](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserNotification.md)
 - [UserNotificationWriteResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserNotificationWriteResponse.md)
 - [UserPresenceData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserPresenceData.md)
 - [UserReactsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserReactsResponse.md)
 - [UserSearchResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserSearchResult.md)
 - [UserSessionInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserSessionInfo.md)
 - [VoteBodyParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteBodyParams.md)
 - [VoteComment_200_response](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteComment_200_response.md)
 - [VoteDeleteResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteDeleteResponse.md)
 - [VoteResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteResponse.md)
 - [VoteResponseUser](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteResponseUser.md)
 - [VoteStyle](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteStyle.md)


<a name="documentation-for-authorization"></a>
## 認証のドキュメント

<a name="api_key"></a>
### api_key

- **タイプ**: API キー
- **API キー パラメータ名**: x-api-key
- **場所**: HTTP ヘッダー