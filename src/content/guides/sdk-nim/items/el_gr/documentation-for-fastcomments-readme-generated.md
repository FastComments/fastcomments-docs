<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *https://fastcomments.com*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *DefaultApi* | **addDomainConfig** | **POST** /api/v1/domain-configs |  |
*DefaultApi* | **addHashTag** | **POST** /api/v1/hash-tags |  |
*DefaultApi* | **addHashTagsBulk** | **POST** /api/v1/hash-tags/bulk |  |
*DefaultApi* | **addPage** | **POST** /api/v1/pages |  |
*DefaultApi* | **addSSOUser** | **POST** /api/v1/sso-users |  |
*DefaultApi* | **aggregate** | **POST** /api/v1/aggregate | Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported. |
*DefaultApi* | **aggregateQuestionResults** | **GET** /api/v1/question-results-aggregation |  |
*DefaultApi* | **blockUserFromComment** | **POST** /api/v1/comments/{id}/block |  |
*DefaultApi* | **bulkAggregateQuestionResults** | **POST** /api/v1/question-results-aggregation/bulk |  |
*DefaultApi* | **changeTicketState** | **PATCH** /api/v1/tickets/{id}/state |  |
*DefaultApi* | **combineCommentsWithQuestionResults** | **GET** /api/v1/question-results-aggregation/combine/comments |  |
*DefaultApi* | **createEmailTemplate** | **POST** /api/v1/email-templates |  |
*DefaultApi* | **createFeedPost** | **POST** /api/v1/feed-posts |  |
*DefaultApi* | **createModerator** | **POST** /api/v1/moderators |  |
*DefaultApi* | **createQuestionConfig** | **POST** /api/v1/question-configs |  |
*DefaultApi* | **createQuestionResult** | **POST** /api/v1/question-results |  |
*DefaultApi* | **createSubscription** | **POST** /api/v1/subscriptions |  |
*DefaultApi* | **createTenant** | **POST** /api/v1/tenants |  |
*DefaultApi* | **createTenantPackage** | **POST** /api/v1/tenant-packages |  |
*DefaultApi* | **createTenantUser** | **POST** /api/v1/tenant-users |  |
*DefaultApi* | **createTicket** | **POST** /api/v1/tickets |  |
*DefaultApi* | **createUserBadge** | **POST** /api/v1/user-badges |  |
*DefaultApi* | **createVote** | **POST** /api/v1/votes |  |
*DefaultApi* | **deleteComment** | **DELETE** /api/v1/comments/{id} |  |
*DefaultApi* | **deleteDomainConfig** | **DELETE** /api/v1/domain-configs/{domain} |  |
*DefaultApi* | **deleteEmailTemplate** | **DELETE** /api/v1/email-templates/{id} |  |
*DefaultApi* | **deleteEmailTemplateRenderError** | **DELETE** /api/v1/email-templates/{id}/render-errors/{errorId} |  |
*DefaultApi* | **deleteHashTag** | **DELETE** /api/v1/hash-tags/{tag} |  |
*DefaultApi* | **deleteModerator** | **DELETE** /api/v1/moderators/{id} |  |
*DefaultApi* | **deleteNotificationCount** | **DELETE** /api/v1/notification-count/{id} |  |
*DefaultApi* | **deletePage** | **DELETE** /api/v1/pages/{id} |  |
*DefaultApi* | **deletePendingWebhookEvent** | **DELETE** /api/v1/pending-webhook-events/{id} |  |
*DefaultApi* | **deleteQuestionConfig** | **DELETE** /api/v1/question-configs/{id} |  |
*DefaultApi* | **deleteQuestionResult** | **DELETE** /api/v1/question-results/{id} |  |
*DefaultApi* | **deleteSSOUser** | **DELETE** /api/v1/sso-users/{id} |  |
*DefaultApi* | **deleteSubscription** | **DELETE** /api/v1/subscriptions/{id} |  |
*DefaultApi* | **deleteTenant** | **DELETE** /api/v1/tenants/{id} |  |
*DefaultApi* | **deleteTenantPackage** | **DELETE** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | **deleteTenantUser** | **DELETE** /api/v1/tenant-users/{id} |  |
*DefaultApi* | **deleteUserBadge** | **DELETE** /api/v1/user-badges/{id} |  |
*DefaultApi* | **deleteVote** | **DELETE** /api/v1/votes/{id} |  |
*DefaultApi* | **flagComment** | **POST** /api/v1/comments/{id}/flag |  |
*DefaultApi* | **getAuditLogs** | **GET** /api/v1/audit-logs |  |
*DefaultApi* | **getCachedNotificationCount** | **GET** /api/v1/notification-count/{id} |  |
*DefaultApi* | **getComment** | **GET** /api/v1/comments/{id} |  |
*DefaultApi* | **getComments** | **GET** /api/v1/comments |  |
*DefaultApi* | **getDomainConfig** | **GET** /api/v1/domain-configs/{domain} |  |
*DefaultApi* | **getDomainConfigs** | **GET** /api/v1/domain-configs |  |
*DefaultApi* | **getEmailTemplate** | **GET** /api/v1/email-templates/{id} |  |
*DefaultApi* | **getEmailTemplateDefinitions** | **GET** /api/v1/email-templates/definitions |  |
*DefaultApi* | **getEmailTemplateRenderErrors** | **GET** /api/v1/email-templates/{id}/render-errors |  |
*DefaultApi* | **getEmailTemplates** | **GET** /api/v1/email-templates |  |
*DefaultApi* | **getFeedPosts** | **GET** /api/v1/feed-posts |  req tenantId afterId |
*DefaultApi* | **getHashTags** | **GET** /api/v1/hash-tags |  |
*DefaultApi* | **getModerator** | **GET** /api/v1/moderators/{id} |  |
*DefaultApi* | **getModerators** | **GET** /api/v1/moderators |  |
*DefaultApi* | **getNotificationCount** | **GET** /api/v1/notifications/count |  |
*DefaultApi* | **getNotifications** | **GET** /api/v1/notifications |  |
*DefaultApi* | **getPageByURLId** | **GET** /api/v1/pages/by-url-id |  |
*DefaultApi* | **getPages** | **GET** /api/v1/pages |  |
*DefaultApi* | **getPendingWebhookEventCount** | **GET** /api/v1/pending-webhook-events/count |  |
*DefaultApi* | **getPendingWebhookEvents** | **GET** /api/v1/pending-webhook-events |  |
*DefaultApi* | **getQuestionConfig** | **GET** /api/v1/question-configs/{id} |  |
*DefaultApi* | **getQuestionConfigs** | **GET** /api/v1/question-configs |  |
*DefaultApi* | **getQuestionResult** | **GET** /api/v1/question-results/{id} |  |
*DefaultApi* | **getQuestionResults** | **GET** /api/v1/question-results |  |
*DefaultApi* | **getSSOUserByEmail** | **GET** /api/v1/sso-users/by-email/{email} |  |
*DefaultApi* | **getSSOUserById** | **GET** /api/v1/sso-users/by-id/{id} |  |
*DefaultApi* | **getSSOUsers** | **GET** /api/v1/sso-users |  |
*DefaultApi* | **getSubscriptions** | **GET** /api/v1/subscriptions |  |
*DefaultApi* | **getTenant** | **GET** /api/v1/tenants/{id} |  |
*DefaultApi* | **getTenantDailyUsages** | **GET** /api/v1/tenant-daily-usage |  |
*DefaultApi* | **getTenantPackage** | **GET** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | **getTenantPackages** | **GET** /api/v1/tenant-packages |  |
*DefaultApi* | **getTenantUser** | **GET** /api/v1/tenant-users/{id} |  |
*DefaultApi* | **getTenantUsers** | **GET** /api/v1/tenant-users |  |
*DefaultApi* | **getTenants** | **GET** /api/v1/tenants |  |
*DefaultApi* | **getTicket** | **GET** /api/v1/tickets/{id} |  |
*DefaultApi* | **getTickets** | **GET** /api/v1/tickets |  |
*DefaultApi* | **getUser** | **GET** /api/v1/users/{id} |  |
*DefaultApi* | **getUserBadge** | **GET** /api/v1/user-badges/{id} |  |
*DefaultApi* | **getUserBadgeProgressById** | **GET** /api/v1/user-badge-progress/{id} |  |
*DefaultApi* | **getUserBadgeProgressByUserId** | **GET** /api/v1/user-badge-progress/user/{userId} |  |
*DefaultApi* | **getUserBadgeProgressList** | **GET** /api/v1/user-badge-progress |  |
*DefaultApi* | **getUserBadges** | **GET** /api/v1/user-badges |  |
*DefaultApi* | **getVotes** | **GET** /api/v1/votes |  |
*DefaultApi* | **getVotesForUser** | **GET** /api/v1/votes/for-user |  |
*DefaultApi* | **patchDomainConfig** | **PATCH** /api/v1/domain-configs/{domainToUpdate} |  |
*DefaultApi* | **patchHashTag** | **PATCH** /api/v1/hash-tags/{tag} |  |
*DefaultApi* | **patchPage** | **PATCH** /api/v1/pages/{id} |  |
*DefaultApi* | **patchSSOUser** | **PATCH** /api/v1/sso-users/{id} |  |
*DefaultApi* | **putDomainConfig** | **PUT** /api/v1/domain-configs/{domainToUpdate} |  |
*DefaultApi* | **putSSOUser** | **PUT** /api/v1/sso-users/{id} |  |
*DefaultApi* | **renderEmailTemplate** | **POST** /api/v1/email-templates/render |  |
*DefaultApi* | **replaceTenantPackage** | **PUT** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | **replaceTenantUser** | **PUT** /api/v1/tenant-users/{id} |  |
*DefaultApi* | **saveComment** | **POST** /api/v1/comments |  |
*DefaultApi* | **saveCommentsBulk** | **POST** /api/v1/comments/bulk |  |
*DefaultApi* | **sendInvite** | **POST** /api/v1/moderators/{id}/send-invite |  |
*DefaultApi* | **sendLoginLink** | **POST** /api/v1/tenant-users/{id}/send-login-link |  |
*DefaultApi* | **unBlockUserFromComment** | **POST** /api/v1/comments/{id}/un-block |  |
*DefaultApi* | **unFlagComment** | **POST** /api/v1/comments/{id}/un-flag |  |
*DefaultApi* | **updateComment** | **PATCH** /api/v1/comments/{id} |  |
*DefaultApi* | **updateEmailTemplate** | **PATCH** /api/v1/email-templates/{id} |  |
*DefaultApi* | **updateFeedPost** | **PATCH** /api/v1/feed-posts/{id} |  |
*DefaultApi* | **updateModerator** | **PATCH** /api/v1/moderators/{id} |  |
*DefaultApi* | **updateNotification** | **PATCH** /api/v1/notifications/{id} |  |
*DefaultApi* | **updateQuestionConfig** | **PATCH** /api/v1/question-configs/{id} |  |
*DefaultApi* | **updateQuestionResult** | **PATCH** /api/v1/question-results/{id} |  |
*DefaultApi* | **updateSubscription** | **PATCH** /api/v1/subscriptions/{id} |  |
*DefaultApi* | **updateTenant** | **PATCH** /api/v1/tenants/{id} |  |
*DefaultApi* | **updateTenantPackage** | **PATCH** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | **updateTenantUser** | **PATCH** /api/v1/tenant-users/{id} |  |
*DefaultApi* | **updateUserBadge** | **PUT** /api/v1/user-badges/{id} |  |
| *ModerationApi* | **deleteModerationVote** | **DELETE** /auth/my-account/moderate-comments/mod_api/vote/{commentId}/{voteId} |  |
*ModerationApi* | **getApiComments** | **GET** /auth/my-account/moderate-comments/mod_api/api/comments |  |
*ModerationApi* | **getApiExportStatus** | **GET** /auth/my-account/moderate-comments/mod_api/api/export/status |  |
*ModerationApi* | **getApiIds** | **GET** /auth/my-account/moderate-comments/mod_api/api/ids |  |
*ModerationApi* | **getBanUsersFromComment** | **GET** /auth/my-account/moderate-comments/mod_api/ban-users/from-comment/{commentId} |  |
*ModerationApi* | **getCommentBanStatus** | **GET** /auth/my-account/moderate-comments/mod_api/get-comment-ban-status/{commentId} |  |
*ModerationApi* | **getCommentChildren** | **GET** /auth/my-account/moderate-comments/mod_api/comment-children/{commentId} |  |
*ModerationApi* | **getCount** | **GET** /auth/my-account/moderate-comments/mod_api/count |  |
*ModerationApi* | **getCounts** | **GET** /auth/my-account/moderate-comments/banned-users/mod_api/counts |  |
*ModerationApi* | **getLogs** | **GET** /auth/my-account/moderate-comments/mod_api/logs/{commentId} |  |
*ModerationApi* | **getManualBadges** | **GET** /auth/my-account/moderate-comments/mod_api/get-manual-badges |  |
*ModerationApi* | **getManualBadgesForUser** | **GET** /auth/my-account/moderate-comments/mod_api/get-manual-badges-for-user |  |
*ModerationApi* | **getModerationComment** | **GET** /auth/my-account/moderate-comments/mod_api/comment/{commentId} |  |
*ModerationApi* | **getModerationCommentText** | **GET** /auth/my-account/moderate-comments/mod_api/get-comment-text/{commentId} |  |
*ModerationApi* | **getPreBanSummary** | **GET** /auth/my-account/moderate-comments/mod_api/pre-ban-summary/{commentId} |  |
*ModerationApi* | **getSearchCommentsSummary** | **GET** /auth/my-account/moderate-comments/mod_api/search/comments/summary |  |
*ModerationApi* | **getSearchPages** | **GET** /auth/my-account/moderate-comments/mod_api/search/pages |  |
*ModerationApi* | **getSearchSites** | **GET** /auth/my-account/moderate-comments/mod_api/search/sites |  |
*ModerationApi* | **getSearchSuggest** | **GET** /auth/my-account/moderate-comments/mod_api/search/suggest |  |
*ModerationApi* | **getSearchUsers** | **GET** /auth/my-account/moderate-comments/mod_api/search/users |  |
*ModerationApi* | **getTrustFactor** | **GET** /auth/my-account/moderate-comments/mod_api/get-trust-factor |  |
*ModerationApi* | **getUserBanPreference** | **GET** /auth/my-account/moderate-comments/mod_api/user-ban-preference |  |
*ModerationApi* | **getUserInternalProfile** | **GET** /auth/my-account/moderate-comments/mod_api/get-user-internal-profile |  |
*ModerationApi* | **postAdjustCommentVotes** | **POST** /auth/my-account/moderate-comments/mod_api/adjust-comment-votes/{commentId} |  |
*ModerationApi* | **postApiExport** | **POST** /auth/my-account/moderate-comments/mod_api/api/export |  |
*ModerationApi* | **postBanUserFromComment** | **POST** /auth/my-account/moderate-comments/mod_api/ban-user/from-comment/{commentId} |  |
*ModerationApi* | **postBanUserUndo** | **POST** /auth/my-account/moderate-comments/mod_api/ban-user/undo |  |
*ModerationApi* | **postBulkPreBanSummary** | **POST** /auth/my-account/moderate-comments/mod_api/bulk-pre-ban-summary |  |
*ModerationApi* | **postCommentsByIds** | **POST** /auth/my-account/moderate-comments/mod_api/comments-by-ids |  |
*ModerationApi* | **postFlagComment** | **POST** /auth/my-account/moderate-comments/mod_api/flag-comment/{commentId} |  |
*ModerationApi* | **postRemoveComment** | **POST** /auth/my-account/moderate-comments/mod_api/remove-comment/{commentId} |  |
*ModerationApi* | **postRestoreDeletedComment** | **POST** /auth/my-account/moderate-comments/mod_api/restore-deleted-comment/{commentId} |  |
*ModerationApi* | **postSetCommentApprovalStatus** | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-approval-status/{commentId} |  |
*ModerationApi* | **postSetCommentReviewStatus** | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-review-status/{commentId} |  |
*ModerationApi* | **postSetCommentSpamStatus** | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-spam-status/{commentId} |  |
*ModerationApi* | **postSetCommentText** | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-text/{commentId} |  |
*ModerationApi* | **postUnFlagComment** | **POST** /auth/my-account/moderate-comments/mod_api/un-flag-comment/{commentId} |  |
*ModerationApi* | **postVote** | **POST** /auth/my-account/moderate-comments/mod_api/vote/{commentId} |  |
*ModerationApi* | **putAwardBadge** | **PUT** /auth/my-account/moderate-comments/mod_api/award-badge |  |
*ModerationApi* | **putCloseThread** | **PUT** /auth/my-account/moderate-comments/mod_api/close-thread |  |
*ModerationApi* | **putRemoveBadge** | **PUT** /auth/my-account/moderate-comments/mod_api/remove-badge |  |
*ModerationApi* | **putReopenThread** | **PUT** /auth/my-account/moderate-comments/mod_api/reopen-thread |  |
*ModerationApi* | **setTrustFactor** | **PUT** /auth/my-account/moderate-comments/mod_api/set-trust-factor |  |
| *PublicApi* | **blockFromCommentPublic** | **POST** /block-from-comment/{commentId} |  |
*PublicApi* | **checkedCommentsForBlocked** | **GET** /check-blocked-comments |  |
*PublicApi* | **createCommentPublic** | **POST** /comments/{tenantId} |  |
*PublicApi* | **createFeedPostPublic** | **POST** /feed-posts/{tenantId} |  |
*PublicApi* | **createV1PageReact** | **POST** /page-reacts/v1/likes/{tenantId} |  |
*PublicApi* | **createV2PageReact** | **POST** /page-reacts/v2/{tenantId} |  |
*PublicApi* | **deleteCommentPublic** | **DELETE** /comments/{tenantId}/{commentId} |  |
*PublicApi* | **deleteCommentVote** | **DELETE** /comments/{tenantId}/{commentId}/vote/{voteId} |  |
*PublicApi* | **deleteFeedPostPublic** | **DELETE** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | **deleteV1PageReact** | **DELETE** /page-reacts/v1/likes/{tenantId} |  |
*PublicApi* | **deleteV2PageReact** | **DELETE** /page-reacts/v2/{tenantId} |  |
*PublicApi* | **flagCommentPublic** | **POST** /flag-comment/{commentId} |  |
*PublicApi* | **getCommentText** | **GET** /comments/{tenantId}/{commentId}/text |  |
*PublicApi* | **getCommentVoteUserNames** | **GET** /comments/{tenantId}/{commentId}/votes |  |
*PublicApi* | **getCommentsForUser** | **GET** /comments-for-user |  |
*PublicApi* | **getCommentsPublic** | **GET** /comments/{tenantId} |  req tenantId urlId |
*PublicApi* | **getEventLog** | **GET** /event-log/{tenantId} |  req tenantId urlId userIdWS |
*PublicApi* | **getFeedPostsPublic** | **GET** /feed-posts/{tenantId} |  req tenantId afterId |
*PublicApi* | **getFeedPostsStats** | **GET** /feed-posts/{tenantId}/stats |  |
*PublicApi* | **getGifLarge** | **GET** /gifs/get-large/{tenantId} |  |
*PublicApi* | **getGifsSearch** | **GET** /gifs/search/{tenantId} |  |
*PublicApi* | **getGifsTrending** | **GET** /gifs/trending/{tenantId} |  |
*PublicApi* | **getGlobalEventLog** | **GET** /event-log/global/{tenantId} |  req tenantId urlId userIdWS |
*PublicApi* | **getOfflineUsers** | **GET** /pages/{tenantId}/users/offline | Past commenters on the page who are NOT currently online. Sorted by displayName. Use this after exhausting /users/online to render a \"Members\" section. Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost. |
*PublicApi* | **getOnlineUsers** | **GET** /pages/{tenantId}/users/online | Currently-online viewers of a page: people whose websocket session is subscribed to the page right now. Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate). |
*PublicApi* | **getPagesPublic** | **GET** /pages/{tenantId} | List pages for a tenant. Used by the FChat desktop client to populate its room list. Requires `enableFChat` to be true on the resolved custom config for each page. Pages that require SSO are filtered against the requesting user's group access. |
*PublicApi* | **getTranslations** | **GET** /translations/{namespace}/{component} |  |
*PublicApi* | **getUserNotificationCount** | **GET** /user-notifications/get-count |  |
*PublicApi* | **getUserNotifications** | **GET** /user-notifications |  |
*PublicApi* | **getUserPresenceStatuses** | **GET** /user-presence-status |  |
*PublicApi* | **getUserReactsPublic** | **GET** /feed-posts/{tenantId}/user-reacts |  |
*PublicApi* | **getUsersInfo** | **GET** /pages/{tenantId}/users/info | Bulk user info for a tenant. Given userIds, return display info from User / SSOUser. Used by the comment widget to enrich users that just appeared via a presence event. No page context: privacy is enforced uniformly (private profiles are masked). |
*PublicApi* | **getV1PageLikes** | **GET** /page-reacts/v1/likes/{tenantId} |  |
*PublicApi* | **getV2PageReactUsers** | **GET** /page-reacts/v2/{tenantId}/list |  |
*PublicApi* | **getV2PageReacts** | **GET** /page-reacts/v2/{tenantId} |  |
*PublicApi* | **lockComment** | **POST** /comments/{tenantId}/{commentId}/lock |  |
*PublicApi* | **logoutPublic** | **PUT** /auth/logout |  |
*PublicApi* | **pinComment** | **POST** /comments/{tenantId}/{commentId}/pin |  |
*PublicApi* | **reactFeedPostPublic** | **POST** /feed-posts/{tenantId}/react/{postId} |  |
*PublicApi* | **resetUserNotificationCount** | **POST** /user-notifications/reset-count |  |
*PublicApi* | **resetUserNotifications** | **POST** /user-notifications/reset |  |
*PublicApi* | **searchUsers** | **GET** /user-search/{tenantId} |  |
*PublicApi* | **setCommentText** | **POST** /comments/{tenantId}/{commentId}/update-text |  |
*PublicApi* | **unBlockCommentPublic** | **DELETE** /block-from-comment/{commentId} |  |
*PublicApi* | **unLockComment** | **POST** /comments/{tenantId}/{commentId}/unlock |  |
*PublicApi* | **unPinComment** | **POST** /comments/{tenantId}/{commentId}/unpin |  |
*PublicApi* | **updateFeedPostPublic** | **PUT** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | **updateUserNotificationCommentSubscriptionStatus** | **POST** /user-notifications/{notificationId}/mark-opted/{optedInOrOut} | Enable or disable notifications for a specific comment. |
*PublicApi* | **updateUserNotificationPageSubscriptionStatus** | **POST** /user-notifications/set-subscription-state/{subscribedOrUnsubscribed} | Enable or disable notifications for a page. When users are subscribed to a page, notifications are created for new root comments, and also |
*PublicApi* | **updateUserNotificationStatus** | **POST** /user-notifications/{notificationId}/mark/{newStatus} |  |
*PublicApi* | **uploadImage** | **POST** /upload-image/{tenantId} | Upload and resize an image |
*PublicApi* | **voteComment** | **POST** /comments/{tenantId}/{commentId}/vote |  |


<a name="documentation-for-models"></a>
## Documentation for Models

 - [APIAuditLog](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIAuditLog.md)
 - [APIBanUserChangeLog](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIBanUserChangeLog.md)
 - [APIBanUserChangedValues](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIBanUserChangedValues.md)
 - [APIBannedUser](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIBannedUser.md)
 - [APIBannedUserWithMultiMatchInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIBannedUserWithMultiMatchInfo.md)
 - [APIComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIComment.md)
 - [APICommentBase](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APICommentBase.md)
 - [APICommentBase_meta](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APICommentBase_meta.md)
 - [APICommentCommonBannedUser](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APICommentCommonBannedUser.md)
 - [APICreateUserBadgeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APICreateUserBadgeResponse.md)
 - [APIDomainConfiguration](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIDomainConfiguration.md)
 - [APIEmptyResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIEmptyResponse.md)
 - [APIEmptySuccessResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIEmptySuccessResponse.md)
 - [APIError](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIError.md)
 - [APIGetCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetCommentResponse.md)
 - [APIGetCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetCommentsResponse.md)
 - [APIGetUserBadgeProgressListResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgeProgressListResponse.md)
 - [APIGetUserBadgeProgressResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgeProgressResponse.md)
 - [APIGetUserBadgeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgeResponse.md)
 - [APIGetUserBadgesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIGetUserBadgesResponse.md)
 - [APIModerateGetUserBanPreferencesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIModerateGetUserBanPreferencesResponse.md)
 - [APIModerateUserBanPreferences](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIModerateUserBanPreferences.md)
 - [APIPage](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIPage.md)
 - [APISSOUser](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APISSOUser.md)
 - [APISaveCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APISaveCommentResponse.md)
 - [APIStatus](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIStatus.md)
 - [APITenant](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APITenant.md)
 - [APITenantDailyUsage](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APITenantDailyUsage.md)
 - [APITicket](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APITicket.md)
 - [APITicketDetail](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APITicketDetail.md)
 - [APITicketFile](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APITicketFile.md)
 - [APIUserSubscription](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/APIUserSubscription.md)
 - [AddDomainConfigParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddDomainConfigParams.md)
 - [AddDomainConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddDomainConfigResponse.md)
 - [AddDomainConfigResponse_anyOf](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddDomainConfigResponse_anyOf.md)
 - [AddPageAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddPageAPIResponse.md)
 - [AddSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AddSSOUserAPIResponse.md)
 - [AdjustCommentVotesParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AdjustCommentVotesParams.md)
 - [AdjustVotesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AdjustVotesResponse.md)
 - [AggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregateQuestionResultsResponse.md)
 - [AggregateResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregateResponse.md)
 - [AggregateTimeBucket](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregateTimeBucket.md)
 - [AggregationAPIError](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationAPIError.md)
 - [AggregationItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationItem.md)
 - [AggregationOpType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationOpType.md)
 - [AggregationOperation](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationOperation.md)
 - [AggregationRequest](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationRequest.md)
 - [AggregationRequest_sort](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationRequest_sort.md)
 - [AggregationResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationResponse.md)
 - [AggregationResponse_stats](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationResponse_stats.md)
 - [AggregationValue](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AggregationValue.md)
 - [AwardUserBadgeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/AwardUserBadgeResponse.md)
 - [BanUserFromCommentResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BanUserFromCommentResult.md)
 - [BanUserUndoParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BanUserUndoParams.md)
 - [BannedUserMatch](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BannedUserMatch.md)
 - [BannedUserMatchType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BannedUserMatchType.md)
 - [BannedUserMatch_matchedOnValue](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BannedUserMatch_matchedOnValue.md)
 - [BillingInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BillingInfo.md)
 - [BlockFromCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BlockFromCommentParams.md)
 - [BlockSuccess](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BlockSuccess.md)
 - [BuildModerationFilterParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BuildModerationFilterParams.md)
 - [BuildModerationFilterResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BuildModerationFilterResponse.md)
 - [BulkAggregateQuestionItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionItem.md)
 - [BulkAggregateQuestionResultsRequest](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionResultsRequest.md)
 - [BulkAggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkAggregateQuestionResultsResponse.md)
 - [BulkCreateHashTagsBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkCreateHashTagsBody.md)
 - [BulkCreateHashTagsBody_tags_inner](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkCreateHashTagsBody_tags_inner.md)
 - [BulkCreateHashTagsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkCreateHashTagsResponse.md)
 - [BulkCreateHashTagsResponse_results_inner](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkCreateHashTagsResponse_results_inner.md)
 - [BulkPreBanParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkPreBanParams.md)
 - [BulkPreBanSummary](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/BulkPreBanSummary.md)
 - [ChangeCommentPinStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ChangeCommentPinStatusResponse.md)
 - [ChangeTicketStateBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ChangeTicketStateBody.md)
 - [ChangeTicketStateResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ChangeTicketStateResponse.md)
 - [CheckBlockedCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CheckBlockedCommentsResponse.md)
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
 - [CommentsByIdsParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CommentsByIdsParams.md)
 - [CreateAPIPageData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateAPIPageData.md)
 - [CreateAPISSOUserData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateAPISSOUserData.md)
 - [CreateAPIUserSubscriptionData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateAPIUserSubscriptionData.md)
 - [CreateCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateCommentParams.md)
 - [CreateEmailTemplateBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateEmailTemplateBody.md)
 - [CreateEmailTemplateResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateEmailTemplateResponse.md)
 - [CreateFeedPostParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostParams.md)
 - [CreateFeedPostResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostResponse.md)
 - [CreateFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateFeedPostsResponse.md)
 - [CreateHashTagBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateHashTagBody.md)
 - [CreateHashTagResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateHashTagResponse.md)
 - [CreateModeratorBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateModeratorBody.md)
 - [CreateModeratorResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateModeratorResponse.md)
 - [CreateQuestionConfigBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateQuestionConfigBody.md)
 - [CreateQuestionConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateQuestionConfigResponse.md)
 - [CreateQuestionResultBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateQuestionResultBody.md)
 - [CreateQuestionResultResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateQuestionResultResponse.md)
 - [CreateSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateSubscriptionAPIResponse.md)
 - [CreateTenantBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTenantBody.md)
 - [CreateTenantPackageBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTenantPackageBody.md)
 - [CreateTenantPackageResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTenantPackageResponse.md)
 - [CreateTenantResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTenantResponse.md)
 - [CreateTenantUserBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTenantUserBody.md)
 - [CreateTenantUserResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTenantUserResponse.md)
 - [CreateTicketBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTicketBody.md)
 - [CreateTicketResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateTicketResponse.md)
 - [CreateUserBadgeParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateUserBadgeParams.md)
 - [CreateV1PageReact](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CreateV1PageReact.md)
 - [CustomConfigParameters](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CustomConfigParameters.md)
 - [CustomEmailTemplate](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/CustomEmailTemplate.md)
 - [DeleteCommentAction](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteCommentAction.md)
 - [DeleteCommentResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteCommentResult.md)
 - [DeleteDomainConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteDomainConfigResponse.md)
 - [DeleteFeedPostPublicResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteFeedPostPublicResponse.md)
 - [DeleteHashTagRequestBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteHashTagRequestBody.md)
 - [DeletePageAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeletePageAPIResponse.md)
 - [DeleteSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteSSOUserAPIResponse.md)
 - [DeleteSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeleteSubscriptionAPIResponse.md)
 - [DeletedCommentResultComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DeletedCommentResultComment.md)
 - [DigestEmailFrequency](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/DigestEmailFrequency.md)
 - [EmailTemplateDefinition](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/EmailTemplateDefinition.md)
 - [EmailTemplateRenderErrorResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/EmailTemplateRenderErrorResponse.md)
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
 - [FlagCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/FlagCommentResponse.md)
 - [GetAuditLogsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetAuditLogsResponse.md)
 - [GetBannedUsersCountResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetBannedUsersCountResponse.md)
 - [GetBannedUsersFromCommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetBannedUsersFromCommentResponse.md)
 - [GetCachedNotificationCountResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCachedNotificationCountResponse.md)
 - [GetCommentBanStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentBanStatusResponse.md)
 - [GetCommentTextResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentTextResponse.md)
 - [GetCommentVoteUserNamesSuccessResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentVoteUserNamesSuccessResponse.md)
 - [GetCommentsForUserResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentsForUserResponse.md)
 - [GetCommentsResponseWithPresence_PublicComment_](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentsResponseWithPresence_PublicComment_.md)
 - [GetCommentsResponse_PublicComment_](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetCommentsResponse_PublicComment_.md)
 - [GetDomainConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigResponse.md)
 - [GetDomainConfigsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigsResponse.md)
 - [GetDomainConfigsResponse_anyOf](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigsResponse_anyOf.md)
 - [GetDomainConfigsResponse_anyOf_1](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetDomainConfigsResponse_anyOf_1.md)
 - [GetEmailTemplateDefinitionsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEmailTemplateDefinitionsResponse.md)
 - [GetEmailTemplateRenderErrorsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEmailTemplateRenderErrorsResponse.md)
 - [GetEmailTemplateResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEmailTemplateResponse.md)
 - [GetEmailTemplatesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEmailTemplatesResponse.md)
 - [GetEventLogResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetEventLogResponse.md)
 - [GetFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetFeedPostsResponse.md)
 - [GetGifsSearchResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetGifsSearchResponse.md)
 - [GetGifsTrendingResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetGifsTrendingResponse.md)
 - [GetHashTagsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetHashTagsResponse.md)
 - [GetModeratorResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetModeratorResponse.md)
 - [GetModeratorsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetModeratorsResponse.md)
 - [GetMyNotificationsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetMyNotificationsResponse.md)
 - [GetNotificationCountResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetNotificationCountResponse.md)
 - [GetNotificationsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetNotificationsResponse.md)
 - [GetPageByURLIdAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPageByURLIdAPIResponse.md)
 - [GetPagesAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPagesAPIResponse.md)
 - [GetPendingWebhookEventCountResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPendingWebhookEventCountResponse.md)
 - [GetPendingWebhookEventsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPendingWebhookEventsResponse.md)
 - [GetPublicFeedPostsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPublicFeedPostsResponse.md)
 - [GetPublicPagesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetPublicPagesResponse.md)
 - [GetQuestionConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetQuestionConfigResponse.md)
 - [GetQuestionConfigsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetQuestionConfigsResponse.md)
 - [GetQuestionResultResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetQuestionResultResponse.md)
 - [GetQuestionResultsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetQuestionResultsResponse.md)
 - [GetSSOUserByEmailAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSSOUserByEmailAPIResponse.md)
 - [GetSSOUserByIdAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSSOUserByIdAPIResponse.md)
 - [GetSSOUsersResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSSOUsersResponse.md)
 - [GetSubscriptionsAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetSubscriptionsAPIResponse.md)
 - [GetTenantDailyUsagesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantDailyUsagesResponse.md)
 - [GetTenantManualBadgesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantManualBadgesResponse.md)
 - [GetTenantPackageResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantPackageResponse.md)
 - [GetTenantPackagesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantPackagesResponse.md)
 - [GetTenantResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantResponse.md)
 - [GetTenantUserResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantUserResponse.md)
 - [GetTenantUsersResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantUsersResponse.md)
 - [GetTenantsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTenantsResponse.md)
 - [GetTicketResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTicketResponse.md)
 - [GetTicketsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTicketsResponse.md)
 - [GetTranslationsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetTranslationsResponse.md)
 - [GetUserInternalProfileResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserInternalProfileResponse.md)
 - [GetUserInternalProfileResponse_profile](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserInternalProfileResponse_profile.md)
 - [GetUserManualBadgesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserManualBadgesResponse.md)
 - [GetUserNotificationCountResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserNotificationCountResponse.md)
 - [GetUserPresenceStatusesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserPresenceStatusesResponse.md)
 - [GetUserResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserResponse.md)
 - [GetUserTrustFactorResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetUserTrustFactorResponse.md)
 - [GetV1PageLikes](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetV1PageLikes.md)
 - [GetV2PageReactUsersResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetV2PageReactUsersResponse.md)
 - [GetV2PageReacts](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetV2PageReacts.md)
 - [GetVotesForUserResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetVotesForUserResponse.md)
 - [GetVotesResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GetVotesResponse.md)
 - [GifGetLargeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GifGetLargeResponse.md)
 - [GifRating](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GifRating.md)
 - [GifSearchInternalError](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GifSearchInternalError.md)
 - [GifSearchResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GifSearchResponse.md)
 - [GifSearchResponse_images_inner_inner](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/GifSearchResponse_images_inner_inner.md)
 - [HeaderAccountNotification](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/HeaderAccountNotification.md)
 - [HeaderState](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/HeaderState.md)
 - [IgnoredResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/IgnoredResponse.md)
 - [ImageContentProfanityLevel](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ImageContentProfanityLevel.md)
 - [ImportedAgentApprovalNotificationFrequency](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ImportedAgentApprovalNotificationFrequency.md)
 - [ImportedSiteType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ImportedSiteType.md)
 - [LiveEvent](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LiveEvent.md)
 - [LiveEventType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LiveEventType.md)
 - [LiveEvent_extraInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/LiveEvent_extraInfo.md)
 - [MediaAsset](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/MediaAsset.md)
 - [MentionAutoCompleteMode](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/MentionAutoCompleteMode.md)
 - [MetaItem](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/MetaItem.md)
 - [ModerationAPIChildCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPIChildCommentsResponse.md)
 - [ModerationAPIComment](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPIComment.md)
 - [ModerationAPICommentLog](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPICommentLog.md)
 - [ModerationAPICommentResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPICommentResponse.md)
 - [ModerationAPICountCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPICountCommentsResponse.md)
 - [ModerationAPIGetCommentIdsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPIGetCommentIdsResponse.md)
 - [ModerationAPIGetCommentsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPIGetCommentsResponse.md)
 - [ModerationAPIGetLogsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationAPIGetLogsResponse.md)
 - [ModerationCommentSearchResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationCommentSearchResponse.md)
 - [ModerationExportResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationExportResponse.md)
 - [ModerationExportStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationExportStatusResponse.md)
 - [ModerationFilter](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationFilter.md)
 - [ModerationPageSearchProjected](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationPageSearchProjected.md)
 - [ModerationPageSearchResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationPageSearchResponse.md)
 - [ModerationSiteSearchProjected](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationSiteSearchProjected.md)
 - [ModerationSiteSearchResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationSiteSearchResponse.md)
 - [ModerationSuggestResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationSuggestResponse.md)
 - [ModerationUserSearchProjected](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationUserSearchProjected.md)
 - [ModerationUserSearchResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ModerationUserSearchResponse.md)
 - [Moderator](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/Moderator.md)
 - [NotificationAndCount](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/NotificationAndCount.md)
 - [NotificationObjectType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/NotificationObjectType.md)
 - [NotificationType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/NotificationType.md)
 - [PageUserEntry](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PageUserEntry.md)
 - [PageUsersInfoResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PageUsersInfoResponse.md)
 - [PageUsersOfflineResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PageUsersOfflineResponse.md)
 - [PageUsersOnlineResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PageUsersOnlineResponse.md)
 - [PagesSortBy](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PagesSortBy.md)
 - [PatchDomainConfigParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchDomainConfigParams.md)
 - [PatchDomainConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchDomainConfigResponse.md)
 - [PatchPageAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchPageAPIResponse.md)
 - [PatchSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PatchSSOUserAPIResponse.md)
 - [PendingCommentToSyncOutbound](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PendingCommentToSyncOutbound.md)
 - [PostRemoveCommentApiResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PostRemoveCommentApiResponse.md)
 - [PreBanSummary](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PreBanSummary.md)
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
 - [PublicPage](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicPage.md)
 - [PublicVote](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PublicVote.md)
 - [PutDomainConfigResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PutDomainConfigResponse.md)
 - [PutSSOUserAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/PutSSOUserAPIResponse.md)
 - [QueryPredicate](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QueryPredicate.md)
 - [QueryPredicate_value](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QueryPredicate_value.md)
 - [QuestionConfig](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionConfig.md)
 - [QuestionConfig_customOptions_inner](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionConfig_customOptions_inner.md)
 - [QuestionDatum](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionDatum.md)
 - [QuestionRenderingType](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionRenderingType.md)
 - [QuestionResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionResult.md)
 - [QuestionResultAggregationOverall](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionResultAggregationOverall.md)
 - [QuestionSubQuestionVisibility](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionSubQuestionVisibility.md)
 - [QuestionWhenSave](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/QuestionWhenSave.md)
 - [ReactBodyParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReactBodyParams.md)
 - [ReactFeedPostResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReactFeedPostResponse.md)
 - [Record_string__before_string_or_null__after_string_or_null___value](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/Record_string__before_string_or_null__after_string_or_null___value.md)
 - [RemoveCommentActionResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RemoveCommentActionResponse.md)
 - [RemoveUserBadgeResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RemoveUserBadgeResponse.md)
 - [RenderEmailTemplateBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RenderEmailTemplateBody.md)
 - [RenderEmailTemplateResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RenderEmailTemplateResponse.md)
 - [RenderableUserNotification](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RenderableUserNotification.md)
 - [RepeatCommentCheckIgnoredReason](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RepeatCommentCheckIgnoredReason.md)
 - [RepeatCommentHandlingAction](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/RepeatCommentHandlingAction.md)
 - [ReplaceTenantPackageBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReplaceTenantPackageBody.md)
 - [ReplaceTenantUserBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ReplaceTenantUserBody.md)
 - [ResetUserNotificationsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/ResetUserNotificationsResponse.md)
 - [SORT_DIR](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SORT_DIR.md)
 - [SSOSecurityLevel](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SSOSecurityLevel.md)
 - [SaveCommentResponseOptimized](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveCommentResponseOptimized.md)
 - [SaveCommentsBulkResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveCommentsBulkResponse.md)
 - [SaveCommentsResponseWithPresence](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SaveCommentsResponseWithPresence.md)
 - [SearchUsersResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SearchUsersResponse.md)
 - [SearchUsersResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SearchUsersResult.md)
 - [SearchUsersSectionedResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SearchUsersSectionedResponse.md)
 - [SetCommentApprovedResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetCommentApprovedResponse.md)
 - [SetCommentTextParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetCommentTextParams.md)
 - [SetCommentTextResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetCommentTextResponse.md)
 - [SetCommentTextResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetCommentTextResult.md)
 - [SetUserTrustFactorResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SetUserTrustFactorResponse.md)
 - [SizePreset](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SizePreset.md)
 - [SortDirections](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SortDirections.md)
 - [SpamRule](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/SpamRule.md)
 - [TOSConfig](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/TOSConfig.md)
 - [TenantBadge](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/TenantBadge.md)
 - [TenantHashTag](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/TenantHashTag.md)
 - [TenantPackage](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/TenantPackage.md)
 - [UnBlockFromCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UnBlockFromCommentParams.md)
 - [UnblockSuccess](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UnblockSuccess.md)
 - [UpdatableCommentParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdatableCommentParams.md)
 - [UpdateAPIPageData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateAPIPageData.md)
 - [UpdateAPISSOUserData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateAPISSOUserData.md)
 - [UpdateAPIUserSubscriptionData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateAPIUserSubscriptionData.md)
 - [UpdateDomainConfigParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateDomainConfigParams.md)
 - [UpdateEmailTemplateBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateEmailTemplateBody.md)
 - [UpdateFeedPostParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateFeedPostParams.md)
 - [UpdateHashTagBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateHashTagBody.md)
 - [UpdateHashTagResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateHashTagResponse.md)
 - [UpdateModeratorBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateModeratorBody.md)
 - [UpdateNotificationBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateNotificationBody.md)
 - [UpdateQuestionConfigBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateQuestionConfigBody.md)
 - [UpdateQuestionResultBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateQuestionResultBody.md)
 - [UpdateSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateSubscriptionAPIResponse.md)
 - [UpdateTenantBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateTenantBody.md)
 - [UpdateTenantPackageBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateTenantPackageBody.md)
 - [UpdateTenantUserBody](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateTenantUserBody.md)
 - [UpdateUserBadgeParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserBadgeParams.md)
 - [UpdateUserNotificationCommentSubscriptionStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserNotificationCommentSubscriptionStatusResponse.md)
 - [UpdateUserNotificationPageSubscriptionStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserNotificationPageSubscriptionStatusResponse.md)
 - [UpdateUserNotificationStatusResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UpdateUserNotificationStatusResponse.md)
 - [UploadImageResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UploadImageResponse.md)
 - [User](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/User.md)
 - [UserBadge](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserBadge.md)
 - [UserBadgeProgress](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserBadgeProgress.md)
 - [UserNotification](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserNotification.md)
 - [UserNotificationCount](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserNotificationCount.md)
 - [UserNotificationWriteResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserNotificationWriteResponse.md)
 - [UserPresenceData](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserPresenceData.md)
 - [UserReactsResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserReactsResponse.md)
 - [UserSearchResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserSearchResult.md)
 - [UserSearchSection](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserSearchSection.md)
 - [UserSearchSectionResult](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserSearchSectionResult.md)
 - [UserSessionInfo](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UserSessionInfo.md)
 - [UsersListLocation](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/UsersListLocation.md)
 - [VoteBodyParams](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteBodyParams.md)
 - [VoteDeleteResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteDeleteResponse.md)
 - [VoteResponse](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteResponse.md)
 - [VoteResponseUser](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteResponseUser.md)
 - [VoteStyle](https://github.com/FastComments/fastcomments-nim/blob/master/docs/Models/VoteStyle.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

<a name="api_key"></a>
### api_key

- **Type**: API key
- **API key parameter name**: x-api-key
- **Location**: HTTP header