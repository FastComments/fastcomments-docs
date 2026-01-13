<a name="documentation-for-api-endpoints"></a>
## Документація по API-ендпоінтам

Всі URI є відносними до *https://fastcomments.com*

| Клас | Метод | HTTP-запит | Опис |
|------------ | ------------- | ------------- | -------------|
| *DefaultApi* | [**addDomainConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addDomainConfig) | **POST** /api/v1/domain-configs |  |
*DefaultApi* | [**addHashTag**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addHashTag) | **POST** /api/v1/hash-tags |  |
*DefaultApi* | [**addHashTagsBulk**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addHashTagsBulk) | **POST** /api/v1/hash-tags/bulk |  |
*DefaultApi* | [**addPage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addPage) | **POST** /api/v1/pages |  |
*DefaultApi* | [**addSSOUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addSSOUser) | **POST** /api/v1/sso-users |  |
*DefaultApi* | [**aggregate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#aggregate) | **POST** /api/v1/aggregate | Агрегує документи шляхом групування їх (якщо groupBy вказано) і застосування кількох операцій. Підтримуються різні операції (наприклад, sum, countDistinct, avg та інші). |
*DefaultApi* | [**aggregateQuestionResults**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#aggregateQuestionResults) | **GET** /api/v1/question-results-aggregation |  |
*DefaultApi* | [**blockUserFromComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#blockUserFromComment) | **POST** /api/v1/comments/{id}/block |  |
*DefaultApi* | [**bulkAggregateQuestionResults**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#bulkAggregateQuestionResults) | **POST** /api/v1/question-results-aggregation/bulk |  |
*DefaultApi* | [**combineCommentsWithQuestionResults**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#combineCommentsWithQuestionResults) | **GET** /api/v1/question-results-aggregation/combine/comments |  |
*DefaultApi* | [**createEmailTemplate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createEmailTemplate) | **POST** /api/v1/email-templates |  |
*DefaultApi* | [**createFeedPost**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createFeedPost) | **POST** /api/v1/feed-posts |  |
*DefaultApi* | [**createModerator**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createModerator) | **POST** /api/v1/moderators |  |
*DefaultApi* | [**createQuestionConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createQuestionConfig) | **POST** /api/v1/question-configs |  |
*DefaultApi* | [**createQuestionResult**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createQuestionResult) | **POST** /api/v1/question-results |  |
*DefaultApi* | [**createSubscription**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createSubscription) | **POST** /api/v1/subscriptions |  |
*DefaultApi* | [**createTenant**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createTenant) | **POST** /api/v1/tenants |  |
*DefaultApi* | [**createTenantPackage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createTenantPackage) | **POST** /api/v1/tenant-packages |  |
*DefaultApi* | [**createTenantUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createTenantUser) | **POST** /api/v1/tenant-users |  |
*DefaultApi* | [**createUserBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createUserBadge) | **POST** /api/v1/user-badges |  |
*DefaultApi* | [**createVote**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createVote) | **POST** /api/v1/votes |  |
*DefaultApi* | [**deleteComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteComment) | **DELETE** /api/v1/comments/{id} |  |
*DefaultApi* | [**deleteDomainConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteDomainConfig) | **DELETE** /api/v1/domain-configs/{domain} |  |
*DefaultApi* | [**deleteEmailTemplate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteEmailTemplate) | **DELETE** /api/v1/email-templates/{id} |  |
*DefaultApi* | [**deleteEmailTemplateRenderError**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteEmailTemplateRenderError) | **DELETE** /api/v1/email-templates/{id}/render-errors/{errorId} |  |
*DefaultApi* | [**deleteHashTag**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteHashTag) | **DELETE** /api/v1/hash-tags/{tag} |  |
*DefaultApi* | [**deleteModerator**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteModerator) | **DELETE** /api/v1/moderators/{id} |  |
*DefaultApi* | [**deleteNotificationCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteNotificationCount) | **DELETE** /api/v1/notification-count/{id} |  |
*DefaultApi* | [**deletePage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deletePage) | **DELETE** /api/v1/pages/{id} |  |
*DefaultApi* | [**deletePendingWebhookEvent**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deletePendingWebhookEvent) | **DELETE** /api/v1/pending-webhook-events/{id} |  |
*DefaultApi* | [**deleteQuestionConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteQuestionConfig) | **DELETE** /api/v1/question-configs/{id} |  |
*DefaultApi* | [**deleteQuestionResult**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteQuestionResult) | **DELETE** /api/v1/question-results/{id} |  |
*DefaultApi* | [**deleteSSOUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteSSOUser) | **DELETE** /api/v1/sso-users/{id} |  |
*DefaultApi* | [**deleteSubscription**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteSubscription) | **DELETE** /api/v1/subscriptions/{id} |  |
*DefaultApi* | [**deleteTenant**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteTenant) | **DELETE** /api/v1/tenants/{id} |  |
*DefaultApi* | [**deleteTenantPackage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteTenantPackage) | **DELETE** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | [**deleteTenantUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteTenantUser) | **DELETE** /api/v1/tenant-users/{id} |  |
*DefaultApi* | [**deleteUserBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteUserBadge) | **DELETE** /api/v1/user-badges/{id} |  |
*DefaultApi* | [**deleteVote**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#deleteVote) | **DELETE** /api/v1/votes/{id} |  |
*DefaultApi* | [**flagComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#flagComment) | **POST** /api/v1/comments/{id}/flag |  |
*DefaultApi* | [**getAuditLogs**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getAuditLogs) | **GET** /api/v1/audit-logs |  |
*DefaultApi* | [**getCachedNotificationCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getCachedNotificationCount) | **GET** /api/v1/notification-count/{id} |  |
*DefaultApi* | [**getComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getComment) | **GET** /api/v1/comments/{id} |  |
*DefaultApi* | [**getComments**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getComments) | **GET** /api/v1/comments |  |
*DefaultApi* | [**getDomainConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getDomainConfig) | **GET** /api/v1/domain-configs/{domain} |  |
*DefaultApi* | [**getDomainConfigs**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getDomainConfigs) | **GET** /api/v1/domain-configs |  |
*DefaultApi* | [**getEmailTemplate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getEmailTemplate) | **GET** /api/v1/email-templates/{id} |  |
*DefaultApi* | [**getEmailTemplateDefinitions**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getEmailTemplateDefinitions) | **GET** /api/v1/email-templates/definitions |  |
*DefaultApi* | [**getEmailTemplateRenderErrors**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getEmailTemplateRenderErrors) | **GET** /api/v1/email-templates/{id}/render-errors |  |
*DefaultApi* | [**getEmailTemplates**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getEmailTemplates) | **GET** /api/v1/email-templates |  |
*DefaultApi* | [**getFeedPosts**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getFeedPosts) | **GET** /api/v1/feed-posts |  req tenantId afterId |
*DefaultApi* | [**getHashTags**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getHashTags) | **GET** /api/v1/hash-tags |  |
*DefaultApi* | [**getModerator**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getModerator) | **GET** /api/v1/moderators/{id} |  |
*DefaultApi* | [**getModerators**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getModerators) | **GET** /api/v1/moderators |  |
*DefaultApi* | [**getNotificationCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getNotificationCount) | **GET** /api/v1/notifications/count |  |
*DefaultApi* | [**getNotifications**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getNotifications) | **GET** /api/v1/notifications |  |
*DefaultApi* | [**getPageByURLId**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getPageByURLId) | **GET** /api/v1/pages/by-url-id |  |
*DefaultApi* | [**getPages**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getPages) | **GET** /api/v1/pages |  |
*DefaultApi* | [**getPendingWebhookEventCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getPendingWebhookEventCount) | **GET** /api/v1/pending-webhook-events/count |  |
*DefaultApi* | [**getPendingWebhookEvents**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getPendingWebhookEvents) | **GET** /api/v1/pending-webhook-events |  |
*DefaultApi* | [**getQuestionConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getQuestionConfig) | **GET** /api/v1/question-configs/{id} |  |
*DefaultApi* | [**getQuestionConfigs**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getQuestionConfigs) | **GET** /api/v1/question-configs |  |
*DefaultApi* | [**getQuestionResult**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getQuestionResult) | **GET** /api/v1/question-results/{id} |  |
*DefaultApi* | [**getQuestionResults**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getQuestionResults) | **GET** /api/v1/question-results |  |
*DefaultApi* | [**getSSOUserByEmail**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getSSOUserByEmail) | **GET** /api/v1/sso-users/by-email/{email} |  |
*DefaultApi* | [**getSSOUserById**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getSSOUserById) | **GET** /api/v1/sso-users/by-id/{id} |  |
*DefaultApi* | [**getSSOUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getSSOUsers) | **GET** /api/v1/sso-users |  |
*DefaultApi* | [**getSubscriptions**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getSubscriptions) | **GET** /api/v1/subscriptions |  |
*DefaultApi* | [**getTenant**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenant) | **GET** /api/v1/tenants/{id} |  |
*DefaultApi* | [**getTenantDailyUsages**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenantDailyUsages) | **GET** /api/v1/tenant-daily-usage |  |
*DefaultApi* | [**getTenantPackage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenantPackage) | **GET** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | [**getTenantPackages**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenantPackages) | **GET** /api/v1/tenant-packages |  |
*DefaultApi* | [**getTenantUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenantUser) | **GET** /api/v1/tenant-users/{id} |  |
*DefaultApi* | [**getTenantUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenantUsers) | **GET** /api/v1/tenant-users |  |
*DefaultApi* | [**getTenants**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTenants) | **GET** /api/v1/tenants |  |
*DefaultApi* | [**getUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getUser) | **GET** /api/v1/users/{id} |  |
*DefaultApi* | [**getUserBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getUserBadge) | **GET** /api/v1/user-badges/{id} |  |
*DefaultApi* | [**getUserBadgeProgressById**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getUserBadgeProgressById) | **GET** /api/v1/user-badge-progress/{id} |  |
*DefaultApi* | [**getUserBadgeProgressByUserId**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getUserBadgeProgressByUserId) | **GET** /api/v1/user-badge-progress/user/{userId} |  |
*DefaultApi* | [**getUserBadgeProgressList**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getUserBadgeProgressList) | **GET** /api/v1/user-badge-progress |  |
*DefaultApi* | [**getUserBadges**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getUserBadges) | **GET** /api/v1/user-badges |  |
*DefaultApi* | [**getVotes**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getVotes) | **GET** /api/v1/votes |  |
*DefaultApi* | [**getVotesForUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getVotesForUser) | **GET** /api/v1/votes/for-user |  |
*DefaultApi* | [**patchDomainConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#patchDomainConfig) | **PATCH** /api/v1/domain-configs/{domainToUpdate} |  |
*DefaultApi* | [**patchHashTag**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#patchHashTag) | **PATCH** /api/v1/hash-tags/{tag} |  |
*DefaultApi* | [**patchPage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#patchPage) | **PATCH** /api/v1/pages/{id} |  |
*DefaultApi* | [**patchSSOUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#patchSSOUser) | **PATCH** /api/v1/sso-users/{id} |  |
*DefaultApi* | [**putDomainConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#putDomainConfig) | **PUT** /api/v1/domain-configs/{domainToUpdate} |  |
*DefaultApi* | [**putSSOUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#putSSOUser) | **PUT** /api/v1/sso-users/{id} |  |
*DefaultApi* | [**renderEmailTemplate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#renderEmailTemplate) | **POST** /api/v1/email-templates/render |  |
*DefaultApi* | [**replaceTenantPackage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#replaceTenantPackage) | **PUT** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | [**replaceTenantUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#replaceTenantUser) | **PUT** /api/v1/tenant-users/{id} |  |
*DefaultApi* | [**saveComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#saveComment) | **POST** /api/v1/comments |  |
*DefaultApi* | [**saveCommentsBulk**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#saveCommentsBulk) | **POST** /api/v1/comments/bulk |  |
*DefaultApi* | [**sendInvite**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#sendInvite) | **POST** /api/v1/moderators/{id}/send-invite |  |
*DefaultApi* | [**sendLoginLink**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#sendLoginLink) | **POST** /api/v1/tenant-users/{id}/send-login-link |  |
*DefaultApi* | [**unBlockUserFromComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#unBlockUserFromComment) | **POST** /api/v1/comments/{id}/un-block |  |
*DefaultApi* | [**unFlagComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#unFlagComment) | **POST** /api/v1/comments/{id}/un-flag |  |
*DefaultApi* | [**updateComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateComment) | **PATCH** /api/v1/comments/{id} |  |
*DefaultApi* | [**updateEmailTemplate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateEmailTemplate) | **PATCH** /api/v1/email-templates/{id} |  |
*DefaultApi* | [**updateFeedPost**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateFeedPost) | **PATCH** /api/v1/feed-posts/{id} |  |
*DefaultApi* | [**updateModerator**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateModerator) | **PATCH** /api/v1/moderators/{id} |  |
*DefaultApi* | [**updateNotification**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateNotification) | **PATCH** /api/v1/notifications/{id} |  |
*DefaultApi* | [**updateQuestionConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateQuestionConfig) | **PATCH** /api/v1/question-configs/{id} |  |
*DefaultApi* | [**updateQuestionResult**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateQuestionResult) | **PATCH** /api/v1/question-results/{id} |  |
*DefaultApi* | [**updateTenant**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateTenant) | **PATCH** /api/v1/tenants/{id} |  |
*DefaultApi* | [**updateTenantPackage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateTenantPackage) | **PATCH** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | [**updateTenantUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateTenantUser) | **PATCH** /api/v1/tenant-users/{id} |  |
*DefaultApi* | [**updateUserBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateUserBadge) | **PUT** /api/v1/user-badges/{id} |  |
| *PublicApi* | [**blockFromCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#blockFromCommentPublic) | **POST** /block-from-comment/{commentId} |  |
*PublicApi* | [**checkedCommentsForBlocked**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#checkedCommentsForBlocked) | **GET** /check-blocked-comments |  |
*PublicApi* | [**createCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#createCommentPublic) | **POST** /comments/{tenantId} |  |
*PublicApi* | [**createFeedPostPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#createFeedPostPublic) | **POST** /feed-posts/{tenantId} |  |
*PublicApi* | [**deleteCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteCommentPublic) | **DELETE** /comments/{tenantId}/{commentId} |  |
*PublicApi* | [**deleteCommentVote**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteCommentVote) | **DELETE** /comments/{tenantId}/{commentId}/vote/{voteId} |  |
*PublicApi* | [**deleteFeedPostPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteFeedPostPublic) | **DELETE** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | [**flagCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#flagCommentPublic) | **POST** /flag-comment/{commentId} |  |
*PublicApi* | [**getCommentText**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentText) | **GET** /comments/{tenantId}/{commentId}/text |  |
*PublicApi* | [**getCommentVoteUserNames**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentVoteUserNames) | **GET** /comments/{tenantId}/{commentId}/votes |  |
*PublicApi* | [**getCommentsPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentsPublic) | **GET** /comments/{tenantId} |  req tenantId urlId |
*PublicApi* | [**getEventLog**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getEventLog) | **GET** /event-log/{tenantId} |  req tenantId urlId userIdWS |
*PublicApi* | [**getFeedPostsPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getFeedPostsPublic) | **GET** /feed-posts/{tenantId} |  req tenantId afterId |
*PublicApi* | [**getFeedPostsStats**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getFeedPostsStats) | **GET** /feed-posts/{tenantId}/stats |  |
*PublicApi* | [**getGlobalEventLog**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getGlobalEventLog) | **GET** /event-log/global/{tenantId} |  req tenantId urlId userIdWS |
*PublicApi* | [**getUserNotificationCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserNotificationCount) | **GET** /user-notifications/get-count |  |
*PublicApi* | [**getUserNotifications**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserNotifications) | **GET** /user-notifications |  |
*PublicApi* | [**getUserPresenceStatuses**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserPresenceStatuses) | **GET** /user-presence-status |  |
*PublicApi* | [**getUserReactsPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserReactsPublic) | **GET** /feed-posts/{tenantId}/user-reacts |  |
*PublicApi* | [**lockComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#lockComment) | **POST** /comments/{tenantId}/{commentId}/lock |  |
*PublicApi* | [**pinComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#pinComment) | **POST** /comments/{tenantId}/{commentId}/pin |  |
*PublicApi* | [**reactFeedPostPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#reactFeedPostPublic) | **POST** /feed-posts/{tenantId}/react/{postId} |  |
*PublicApi* | [**resetUserNotificationCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#resetUserNotificationCount) | **POST** /user-notifications/reset-count |  |
*PublicApi* | [**resetUserNotifications**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#resetUserNotifications) | **POST** /user-notifications/reset |  |
*PublicApi* | [**searchUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#searchUsers) | **GET** /user-search/{tenantId} |  |
*PublicApi* | [**setCommentText**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#setCommentText) | **POST** /comments/{tenantId}/{commentId}/update-text |  |
*PublicApi* | [**unBlockCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#unBlockCommentPublic) | **DELETE** /block-from-comment/{commentId} |  |
*PublicApi* | [**unLockComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#unLockComment) | **POST** /comments/{tenantId}/{commentId}/unlock |  |
*PublicApi* | [**unPinComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#unPinComment) | **POST** /comments/{tenantId}/{commentId}/unpin |  |
*PublicApi* | [**updateFeedPostPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateFeedPostPublic) | **PUT** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | [**updateUserNotificationCommentSubscriptionStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateUserNotificationCommentSubscriptionStatus) | **POST** /user-notifications/{notificationId}/mark-opted/{optedInOrOut} | Увімкнути або вимкнути сповіщення для окремого коментаря. |
*PublicApi* | [**updateUserNotificationPageSubscriptionStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateUserNotificationPageSubscriptionStatus) | **POST** /user-notifications/set-subscription-state/{subscribedOrUnsubscribed} | Увімкнути або вимкнути сповіщення для сторінки. Коли користувачі підписані на сторінку, сповіщення створюються для нових кореневих коментарів, а також |
*PublicApi* | [**updateUserNotificationStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateUserNotificationStatus) | **POST** /user-notifications/{notificationId}/mark/{newStatus} |  |
*PublicApi* | [**uploadImage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#uploadImage) | **POST** /upload-image/{tenantId} | Завантажити та змінити розмір зображення |
*PublicApi* | [**voteComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#voteComment) | **POST** /comments/{tenantId}/{commentId}/vote |  |


<a name="documentation-for-models"></a>
## Документація по моделях

 - [APIAuditLog](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIAuditLog.md)
 - [APIComment](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIComment.md)
 - [APICommentBase](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APICommentBase.md)
 - [APICreateUserBadgeResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APICreateUserBadgeResponse.md)
 - [APIDomainConfiguration](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIDomainConfiguration.md)
 - [APIEmptyResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIEmptyResponse.md)
 - [APIEmptySuccessResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIEmptySuccessResponse.md)
 - [APIError](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIError.md)
 - [APIGetCommentResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIGetCommentResponse.md)
 - [APIGetCommentsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIGetCommentsResponse.md)
 - [APIGetUserBadgeProgressListResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIGetUserBadgeProgressListResponse.md)
 - [APIGetUserBadgeProgressResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIGetUserBadgeProgressResponse.md)
 - [APIGetUserBadgeResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIGetUserBadgeResponse.md)
 - [APIGetUserBadgesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIGetUserBadgesResponse.md)
 - [APIPage](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIPage.md)
 - [APISSOUser](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APISSOUser.md)
 - [APIStatus](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIStatus.md)
 - [APITenant](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITenant.md)
 - [APITenantDailyUsage](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITenantDailyUsage.md)
 - [APIUserSubscription](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIUserSubscription.md)
 - [AddDomainConfigParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddDomainConfigParams.md)
 - [AddDomainConfig_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddDomainConfig_200_response.md)
 - [AddDomainConfig_200_response_anyOf](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddDomainConfig_200_response_anyOf.md)
 - [AddHashTag_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddHashTag_200_response.md)
 - [AddHashTagsBulk_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddHashTagsBulk_200_response.md)
 - [AddPageAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddPageAPIResponse.md)
 - [AddSSOUserAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddSSOUserAPIResponse.md)
 - [AggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregateQuestionResultsResponse.md)
 - [AggregateQuestionResults_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregateQuestionResults_200_response.md)
 - [AggregateTimeBucket](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregateTimeBucket.md)
 - [AggregationItem](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationItem.md)
 - [AggregationOpType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationOpType.md)
 - [AggregationOperation](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationOperation.md)
 - [AggregationRequest](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationRequest.md)
 - [AggregationRequest_sort](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationRequest_sort.md)
 - [AggregationResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationResponse.md)
 - [AggregationResponse_stats](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationResponse_stats.md)
 - [AggregationValue](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationValue.md)
 - [BillingInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BillingInfo.md)
 - [BlockFromCommentParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BlockFromCommentParams.md)
 - [BlockFromCommentPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BlockFromCommentPublic_200_response.md)
 - [BlockSuccess](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BlockSuccess.md)
 - [BulkAggregateQuestionItem](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkAggregateQuestionItem.md)
 - [BulkAggregateQuestionResultsRequest](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkAggregateQuestionResultsRequest.md)
 - [BulkAggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkAggregateQuestionResultsResponse.md)
 - [BulkAggregateQuestionResults_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkAggregateQuestionResults_200_response.md)
 - [BulkCreateHashTagsBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkCreateHashTagsBody.md)
 - [BulkCreateHashTagsBody_tags_inner](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkCreateHashTagsBody_tags_inner.md)
 - [BulkCreateHashTagsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BulkCreateHashTagsResponse.md)
 - [ChangeCommentPinStatusResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/ChangeCommentPinStatusResponse.md)
 - [CheckBlockedCommentsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CheckBlockedCommentsResponse.md)
 - [CheckedCommentsForBlocked_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CheckedCommentsForBlocked_200_response.md)
 - [CombineCommentsWithQuestionResults_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CombineCommentsWithQuestionResults_200_response.md)
 - [CombineQuestionResultsWithCommentsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CombineQuestionResultsWithCommentsResponse.md)
 - [CommentData](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentData.md)
 - [CommentHTMLRenderingMode](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentHTMLRenderingMode.md)
 - [CommentLogData](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentLogData.md)
 - [CommentLogEntry](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentLogEntry.md)
 - [CommentLogType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentLogType.md)
 - [CommentQuestionResultsRenderingType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentQuestionResultsRenderingType.md)
 - [CommentQuestionsRequired](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentQuestionsRequired.md)
 - [CommentTextUpdateRequest](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentTextUpdateRequest.md)
 - [CommentThreadDeletionMode](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentThreadDeletionMode.md)
 - [CommentUserBadgeInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentUserBadgeInfo.md)
 - [CommentUserHashTagInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentUserHashTagInfo.md)
 - [CommentUserMentionInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommentUserMentionInfo.md)
 - [CommenterNameFormats](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CommenterNameFormats.md)
 - [CreateAPIPageData](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateAPIPageData.md)
 - [CreateAPISSOUserData](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateAPISSOUserData.md)
 - [CreateAPIUserSubscriptionData](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateAPIUserSubscriptionData.md)
 - [CreateCommentParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateCommentParams.md)
 - [CreateCommentPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateCommentPublic_200_response.md)
 - [CreateEmailTemplateBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateEmailTemplateBody.md)
 - [CreateEmailTemplateResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateEmailTemplateResponse.md)
 - [CreateEmailTemplate_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateEmailTemplate_200_response.md)
 - [CreateFeedPostParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateFeedPostParams.md)
 - [CreateFeedPostPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateFeedPostPublic_200_response.md)
 - [CreateFeedPostResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateFeedPostResponse.md)
 - [CreateFeedPost_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateFeedPost_200_response.md)
 - [CreateFeedPostsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateFeedPostsResponse.md)
 - [CreateHashTagBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateHashTagBody.md)
 - [CreateHashTagResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateHashTagResponse.md)
 - [CreateModeratorBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateModeratorBody.md)
 - [CreateModeratorResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateModeratorResponse.md)
 - [CreateModerator_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateModerator_200_response.md)
 - [CreateQuestionConfigBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateQuestionConfigBody.md)
 - [CreateQuestionConfigResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateQuestionConfigResponse.md)
 - [CreateQuestionConfig_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateQuestionConfig_200_response.md)
 - [CreateQuestionResultBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateQuestionResultBody.md)
 - [CreateQuestionResultResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateQuestionResultResponse.md)
 - [CreateQuestionResult_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateQuestionResult_200_response.md)
 - [CreateSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateSubscriptionAPIResponse.md)
 - [CreateTenantBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantBody.md)
 - [CreateTenantPackageBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantPackageBody.md)
 - [CreateTenantPackageResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantPackageResponse.md)
 - [CreateTenantPackage_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantPackage_200_response.md)
 - [CreateTenantResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantResponse.md)
 - [CreateTenantUserBody](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantUserBody.md)
 - [CreateTenantUserResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantUserResponse.md)
 - [CreateTenantUser_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenantUser_200_response.md)
 - [CreateTenant_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateTenant_200_response.md)
 - [CreateUserBadgeParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateUserBadgeParams.md)
 - [CreateUserBadge_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CreateUserBadge_200_response.md)
 - [CustomConfigParameters](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CustomConfigParameters.md)
 - [CustomEmailTemplate](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/CustomEmailTemplate.md)
 - [DeleteCommentAction](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteCommentAction.md)
 - [DeleteCommentPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteCommentPublic_200_response.md)
 - [DeleteCommentResult](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteCommentResult.md)
 - [DeleteCommentVote_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteCommentVote_200_response.md)
 - [DeleteComment_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteComment_200_response.md)
 - [DeleteDomainConfig_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteDomainConfig_200_response.md)
 - [DeleteFeedPostPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteFeedPostPublic_200_response.md)
 - [DeleteFeedPostPublic_200_response_anyOf](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteFeedPostPublic_200_response_anyOf.md)
 - [DeleteHashTag_request](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteHashTag_request.md)
 - [DeletePageAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeletePageAPIResponse.md)
 - [DeleteSSOUserAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteSSOUserAPIResponse.md)
 - [DeleteSubscriptionAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeleteSubscriptionAPIResponse.md)
 - [DeletedCommentResultComment](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DeletedCommentResultComment.md)
 - [DigestEmailFrequency](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/DigestEmailFrequency.md)
 - [EmailTemplateDefinition](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/EmailTemplateDefinition.md)
 - [EmailTemplateRenderErrorResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/EmailTemplateRenderErrorResponse.md)
 - [EventLogEntry](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/EventLogEntry.md)
 - [FComment](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FComment.md)
 - [FComment_meta](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FComment_meta.md)
 - [FeedPost](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FeedPost.md)
 - [FeedPostLink](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FeedPostLink.md)
 - [FeedPostMediaItem](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FeedPostMediaItem.md)
 - [FeedPostMediaItemAsset](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FeedPostMediaItemAsset.md)
 - [FeedPostStats](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FeedPostStats.md)
 - [FeedPostsStatsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FeedPostsStatsResponse.md)
 - [FindCommentsByRangeItem](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FindCommentsByRangeItem.md)
 - [FindCommentsByRangeResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FindCommentsByRangeResponse.md)
 - [FlagCommentPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FlagCommentPublic_200_response.md)
 - [FlagCommentResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FlagCommentResponse.md)
 - [FlagComment_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/FlagComment_200_response.md)
 - [GetAuditLogsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetAuditLogsResponse.md)
 - [GetAuditLogs_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetAuditLogs_200_response.md)
 - [GetCachedNotificationCountResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCachedNotificationCountResponse.md)
 - [GetCachedNotificationCount_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCachedNotificationCount_200_response.md)
 - [GetCommentText_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCommentText_200_response.md)
 - [GetCommentVoteUserNamesSuccessResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCommentVoteUserNamesSuccessResponse.md)
 - [GetCommentVoteUserNames_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCommentVoteUserNames_200_response.md)
 - [GetComment_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetComment_200_response.md)
 - [GetCommentsPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCommentsPublic_200_response.md)
 - [GetCommentsResponseWithPresence_PublicComment_](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCommentsResponseWithPresence_PublicComment_.md)
 - [GetCommentsResponse_PublicComment_](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetCommentsResponse_PublicComment_.md)
 - [GetComments_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetComments_200_response.md)
 - [GetDomainConfig_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetDomainConfig_200_response.md)
 - [GetDomainConfigs_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetDomainConfigs_200_response.md)
 - [GetDomainConfigs_200_response_anyOf](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetDomainConfigs_200_response_anyOf.md)
 - [GetDomainConfigs_200_response_anyOf_1](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetDomainConfigs_200_response_anyOf_1.md)
 - [GetEmailTemplateDefinitionsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplateDefinitionsResponse.md)
 - [GetEmailTemplateDefinitions_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplateDefinitions_200_response.md)
 - [GetEmailTemplateRenderErrorsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplateRenderErrorsResponse.md)
 - [GetEmailTemplateRenderErrors_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplateRenderErrors_200_response.md)
 - [GetEmailTemplateResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplateResponse.md)
 - [GetEmailTemplate_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplate_200_response.md)
 - [GetEmailTemplatesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplatesResponse.md)
 - [GetEmailTemplates_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEmailTemplates_200_response.md)
 - [GetEventLogResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEventLogResponse.md)
 - [GetEventLog_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetEventLog_200_response.md)
 - [GetFeedPostsPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetFeedPostsPublic_200_response.md)
 - [GetFeedPostsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetFeedPostsResponse.md)
 - [GetFeedPostsStats_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetFeedPostsStats_200_response.md)
 - [GetFeedPosts_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetFeedPosts_200_response.md)
 - [GetHashTagsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetHashTagsResponse.md)
 - [GetHashTags_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetHashTags_200_response.md)
 - [GetModeratorResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetModeratorResponse.md)
 - [GetModerator_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetModerator_200_response.md)
 - [GetModeratorsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetModeratorsResponse.md)
 - [GetModerators_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetModerators_200_response.md)
 - [GetMyNotificationsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetMyNotificationsResponse.md)
 - [GetNotificationCountResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetNotificationCountResponse.md)
 - [GetNotificationCount_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetNotificationCount_200_response.md)
 - [GetNotificationsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetNotificationsResponse.md)
 - [GetNotifications_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetNotifications_200_response.md)
 - [GetPageByURLIdAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPageByURLIdAPIResponse.md)
 - [GetPagesAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPagesAPIResponse.md)
 - [GetPendingWebhookEventCountResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPendingWebhookEventCountResponse.md)
 - [GetPendingWebhookEventCount_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPendingWebhookEventCount_200_response.md)
 - [GetPendingWebhookEventsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPendingWebhookEventsResponse.md)
 - [GetPendingWebhookEvents_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPendingWebhookEvents_200_response.md)
 - [GetPublicFeedPostsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetPublicFeedPostsResponse.md)
 - [GetQuestionConfigResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionConfigResponse.md)
 - [GetQuestionConfig_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionConfig_200_response.md)
 - [GetQuestionConfigsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionConfigsResponse.md)
 - [GetQuestionConfigs_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionConfigs_200_response.md)
 - [GetQuestionResultResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionResultResponse.md)
 - [GetQuestionResult_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionResult_200_response.md)
 - [GetQuestionResultsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionResultsResponse.md)
 - [GetQuestionResults_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetQuestionResults_200_response.md)
 - [GetSSOUserByEmailAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetSSOUserByEmailAPIResponse.md)
 - [GetSSOUserByIdAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetSSOUserByIdAPIResponse.md)
 - [GetSSOUsers_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetSSOUsers_200_response.md)
 - [GetSubscriptionsAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetSubscriptionsAPIResponse.md)
 - [GetTenantDailyUsagesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantDailyUsagesResponse.md)
 - [GetTenantDailyUsages_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantDailyUsages_200_response.md)
 - [GetTenantPackageResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantPackageResponse.md)
 - [GetTenantPackage_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantPackage_200_response.md)
 - [GetTenantPackagesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantPackagesResponse.md)
 - [GetTenantPackages_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantPackages_200_response.md)
 - [GetTenantResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantResponse.md)
 - [GetTenantUserResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantUserResponse.md)
 - [GetTenantUser_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantUser_200_response.md)
 - [GetTenantUsersResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantUsersResponse.md)
 - [GetTenantUsers_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantUsers_200_response.md)
 - [GetTenant_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenant_200_response.md)
 - [GetTenantsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenantsResponse.md)
 - [GetTenants_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetTenants_200_response.md)
 - [GetUserBadgeProgressById_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserBadgeProgressById_200_response.md)
 - [GetUserBadgeProgressList_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserBadgeProgressList_200_response.md)
 - [GetUserBadge_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserBadge_200_response.md)
 - [GetUserBadges_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserBadges_200_response.md)
 - [GetUserNotificationCountResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserNotificationCountResponse.md)
 - [GetUserNotificationCount_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserNotificationCount_200_response.md)
 - [GetUserNotifications_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserNotifications_200_response.md)
 - [GetUserPresenceStatusesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserPresenceStatusesResponse.md)
 - [GetUserPresenceStatuses_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserPresenceStatuses_200_response.md)
 - [GetUserReactsPublic_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserReactsPublic_200_response.md)
 - [GetUserResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUserResponse.md)
 - [GetUser_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetUser_200_response.md)
 - [GetVotesForUserResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetVotesForUserResponse.md)
 - [GetVotesForUser_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetVotesForUser_200_response.md)
 - [GetVotesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetVotesResponse.md)
 - [GetVotes_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GetVotes_200_response.md)
 - [GifRating](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/GifRating.md)
 - [HeaderState](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/HeaderState.md)
 - [IgnoredResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/IgnoredResponse.md)
 - [ImageContentProfanityLevel](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/ImageContentProfanityLevel.md)
 - [ImportedSiteType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/ImportedSiteType.md)
 - [LiveEvent](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/LiveEvent.md)
 - [LiveEventType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/LiveEventType.md)
 - [LiveEvent_extraInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/LiveEvent_extraInfo.md)
 - [LockComment_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/LockComment_200_response.md)
 - [MediaAsset](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/MediaAsset.md)
 - [MetaItem](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/MetaItem.md)
 - [Moderator](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/Moderator.md)
 - [NotificationAndCount](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/NotificationAndCount.md)
 - [NotificationObjectType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/NotificationObjectType.md)
 - [NotificationType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/NotificationType.md)
 - [PatchDomainConfigParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PatchDomainConfigParams.md)
 - [PatchHashTag_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PatchHashTag_200_response.md)
 - [PatchPageAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PatchPageAPIResponse.md)
 - [PatchSSOUserAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PatchSSOUserAPIResponse.md)
 - [PendingCommentToSyncOutbound](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PendingCommentToSyncOutbound.md)
 - [PinComment_200_response](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PinComment_200_response.md)
 - [PubSubComment](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PubSubComment.md)
 - [PubSubCommentBase](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PubSubCommentBase.md)
 - [PubSubVote](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PubSubVote.md)
 - [PublicAPIDeleteCommentResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PublicAPIDeleteCommentResponse.md)
 - [PublicAPIGetCommentTextResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PublicAPIGetCommentTextResponse.md)
 - [PublicAPISetCommentTextResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/PublicAPISetCommentTextResponse