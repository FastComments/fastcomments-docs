<a name="documentation-for-api-endpoints"></a>
## Документація для кінцевих точок API

All URIs are relative to *https://fastcomments.com*

| Клас | Метод | HTTP‑запит | Опис |
|------------ | ------------- | ------------- | -------------|
| *DefaultApi* | [**addDomainConfig**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addDomainConfig) | **POST** /api/v1/domain-configs |  |
*DefaultApi* | [**addHashTag**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addHashTag) | **POST** /api/v1/hash-tags |  |
*DefaultApi* | [**addHashTagsBulk**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addHashTagsBulk) | **POST** /api/v1/hash-tags/bulk |  |
*DefaultApi* | [**addPage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addPage) | **POST** /api/v1/pages |  |
*DefaultApi* | [**addSSOUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#addSSOUser) | **POST** /api/v1/sso-users |  |
*DefaultApi* | [**aggregate**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#aggregate) | **POST** /api/v1/aggregate | Агрегує документи, групуючи їх (якщо вказано groupBy) та застосовуючи кілька операцій. Підтримуються різні операції (наприклад, sum, countDistinct, avg тощо). |
*DefaultApi* | [**aggregateQuestionResults**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#aggregateQuestionResults) | **GET** /api/v1/question-results-aggregation |  |
*DefaultApi* | [**blockUserFromComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#blockUserFromComment) | **POST** /api/v1/comments/{id}/block |  |
*DefaultApi* | [**bulkAggregateQuestionResults**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#bulkAggregateQuestionResults) | **POST** /api/v1/question-results-aggregation/bulk |  |
*DefaultApi* | [**changeTicketState**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#changeTicketState) | **PATCH** /api/v1/tickets/{id}/state |  |
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
*DefaultApi* | [**createTicket**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#createTicket) | **POST** /api/v1/tickets |  |
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
*DefaultApi* | [**getTicket**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTicket) | **GET** /api/v1/tickets/{id} |  |
*DefaultApi* | [**getTickets**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#getTickets) | **GET** /api/v1/tickets |  |
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
*DefaultApi* | [**updateSubscription**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateSubscription) | **PATCH** /api/v1/subscriptions/{id} |  |
*DefaultApi* | [**updateTenant**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateTenant) | **PATCH** /api/v1/tenants/{id} |  |
*DefaultApi* | [**updateTenantPackage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateTenantPackage) | **PATCH** /api/v1/tenant-packages/{id} |  |
*DefaultApi* | [**updateTenantUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateTenantUser) | **PATCH** /api/v1/tenant-users/{id} |  |
*DefaultApi* | [**updateUserBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/DefaultApi.md#updateUserBadge) | **PUT** /api/v1/user-badges/{id} |  |
| *ModerationApi* | [**deleteModerationVote**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#deleteModerationVote) | **DELETE** /auth/my-account/moderate-comments/mod_api/vote/{commentId}/{voteId} |  |
*ModerationApi* | [**getApiComments**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getApiComments) | **GET** /auth/my-account/moderate-comments/mod_api/api/comments |  |
*ModerationApi* | [**getApiExportStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getApiExportStatus) | **GET** /auth/my-account/moderate-comments/mod_api/api/export/status |  |
*ModerationApi* | [**getApiIds**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getApiIds) | **GET** /auth/my-account/moderate-comments/mod_api/api/ids |  |
*ModerationApi* | [**getBanUsersFromComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getBanUsersFromComment) | **GET** /auth/my-account/moderate-comments/mod_api/ban-users/from-comment/{commentId} |  |
*ModerationApi* | [**getCommentBanStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getCommentBanStatus) | **GET** /auth/my-account/moderate-comments/mod_api/get-comment-ban-status/{commentId} |  |
*ModerationApi* | [**getCommentChildren**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getCommentChildren) | **GET** /auth/my-account/moderate-comments/mod_api/comment-children/{commentId} |  |
*ModerationApi* | [**getCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getCount) | **GET** /auth/my-account/moderate-comments/mod_api/count |  |
*ModerationApi* | [**getCounts**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getCounts) | **GET** /auth/my-account/moderate-comments/banned-users/mod_api/counts |  |
*ModerationApi* | [**getLogs**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getLogs) | **GET** /auth/my-account/moderate-comments/mod_api/logs/{commentId} |  |
*ModerationApi* | [**getManualBadges**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getManualBadges) | **GET** /auth/my-account/moderate-comments/mod_api/get-manual-badges |  |
*ModerationApi* | [**getManualBadgesForUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getManualBadgesForUser) | **GET** /auth/my-account/moderate-comments/mod_api/get-manual-badges-for-user |  |
*ModerationApi* | [**getModerationComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getModerationComment) | **GET** /auth/my-account/moderate-comments/mod_api/comment/{commentId} |  |
*ModerationApi* | [**getModerationCommentText**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getModerationCommentText) | **GET** /auth/my-account/moderate-comments/mod_api/get-comment-text/{commentId} |  |
*ModerationApi* | [**getPreBanSummary**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getPreBanSummary) | **GET** /auth/my-account/moderate-comments/mod_api/pre-ban-summary/{commentId} |  |
*ModerationApi* | [**getSearchCommentsSummary**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getSearchCommentsSummary) | **GET** /auth/my-account/moderate-comments/mod_api/search/comments/summary |  |
*ModerationApi* | [**getSearchPages**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getSearchPages) | **GET** /auth/my-account/moderate-comments/mod_api/search/pages |  |
*ModerationApi* | [**getSearchSites**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getSearchSites) | **GET** /auth/my-account/moderate-comments/mod_api/search/sites |  |
*ModerationApi* | [**getSearchSuggest**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getSearchSuggest) | **GET** /auth/my-account/moderate-comments/mod_api/search/suggest |  |
*ModerationApi* | [**getSearchUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getSearchUsers) | **GET** /auth/my-account/moderate-comments/mod_api/search/users |  |
*ModerationApi* | [**getTrustFactor**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getTrustFactor) | **GET** /auth/my-account/moderate-comments/mod_api/get-trust-factor |  |
*ModerationApi* | [**getUserBanPreference**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getUserBanPreference) | **GET** /auth/my-account/moderate-comments/mod_api/user-ban-preference |  |
*ModerationApi* | [**getUserInternalProfile**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#getUserInternalProfile) | **GET** /auth/my-account/moderate-comments/mod_api/get-user-internal-profile |  |
*ModerationApi* | [**postAdjustCommentVotes**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postAdjustCommentVotes) | **POST** /auth/my-account/moderate-comments/mod_api/adjust-comment-votes/{commentId} |  |
*ModerationApi* | [**postApiExport**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postApiExport) | **POST** /auth/my-account/moderate-comments/mod_api/api/export |  |
*ModerationApi* | [**postBanUserFromComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postBanUserFromComment) | **POST** /auth/my-account/moderate-comments/mod_api/ban-user/from-comment/{commentId} |  |
*ModerationApi* | [**postBanUserUndo**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postBanUserUndo) | **POST** /auth/my-account/moderate-comments/mod_api/ban-user/undo |  |
*ModerationApi* | [**postBulkPreBanSummary**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postBulkPreBanSummary) | **POST** /auth/my-account/moderate-comments/mod_api/bulk-pre-ban-summary |  |
*ModerationApi* | [**postCommentsByIds**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postCommentsByIds) | **POST** /auth/my-account/moderate-comments/mod_api/comments-by-ids |  |
*ModerationApi* | [**postFlagComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postFlagComment) | **POST** /auth/my-account/moderate-comments/mod_api/flag-comment/{commentId} |  |
*ModerationApi* | [**postRemoveComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postRemoveComment) | **POST** /auth/my-account/moderate-comments/mod_api/remove-comment/{commentId} |  |
*ModerationApi* | [**postRestoreDeletedComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postRestoreDeletedComment) | **POST** /auth/my-account/moderate-comments/mod_api/restore-deleted-comment/{commentId} |  |
*ModerationApi* | [**postSetCommentApprovalStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postSetCommentApprovalStatus) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-approval-status/{commentId} |  |
*ModerationApi* | [**postSetCommentReviewStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postSetCommentReviewStatus) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-review-status/{commentId} |  |
*ModerationApi* | [**postSetCommentSpamStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postSetCommentSpamStatus) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-spam-status/{commentId} |  |
*ModerationApi* | [**postSetCommentText**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postSetCommentText) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-text/{commentId} |  |
*ModerationApi* | [**postUnFlagComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postUnFlagComment) | **POST** /auth/my-account/moderate-comments/mod_api/un-flag-comment/{commentId} |  |
*ModerationApi* | [**postVote**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#postVote) | **POST** /auth/my-account/moderate-comments/mod_api/vote/{commentId} |  |
*ModerationApi* | [**putAwardBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#putAwardBadge) | **PUT** /auth/my-account/moderate-comments/mod_api/award-badge |  |
*ModerationApi* | [**putCloseThread**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#putCloseThread) | **PUT** /auth/my-account/moderate-comments/mod_api/close-thread |  |
*ModerationApi* | [**putRemoveBadge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#putRemoveBadge) | **PUT** /auth/my-account/moderate-comments/mod_api/remove-badge |  |
*ModerationApi* | [**putReopenThread**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#putReopenThread) | **PUT** /auth/my-account/moderate-comments/mod_api/reopen-thread |  |
*ModerationApi* | [**setTrustFactor**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/ModerationApi.md#setTrustFactor) | **PUT** /auth/my-account/moderate-comments/mod_api/set-trust-factor |  |
| *PublicApi* | [**blockFromCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#blockFromCommentPublic) | **POST** /block-from-comment/{commentId} |  |
*PublicApi* | [**checkedCommentsForBlocked**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#checkedCommentsForBlocked) | **GET** /check-blocked-comments |  |
*PublicApi* | [**createCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#createCommentPublic) | **POST** /comments/{tenantId} |  |
*PublicApi* | [**createFeedPostPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#createFeedPostPublic) | **POST** /feed-posts/{tenantId} |  |
*PublicApi* | [**createV1PageReact**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#createV1PageReact) | **POST** /page-reacts/v1/likes/{tenantId} |  |
*PublicApi* | [**createV2PageReact**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#createV2PageReact) | **POST** /page-reacts/v2/{tenantId} |  |
*PublicApi* | [**deleteCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteCommentPublic) | **DELETE** /comments/{tenantId}/{commentId} |  |
*PublicApi* | [**deleteCommentVote**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteCommentVote) | **DELETE** /comments/{tenantId}/{commentId}/vote/{voteId} |  |
*PublicApi* | [**deleteFeedPostPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteFeedPostPublic) | **DELETE** /feed-posts/{tenantId}/{postId} |  |
*PublicApi* | [**deleteV1PageReact**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteV1PageReact) | **DELETE** /page-reacts/v1/likes/{tenantId} |  |
*PublicApi* | [**deleteV2PageReact**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#deleteV2PageReact) | **DELETE** /page-reacts/v2/{tenantId} |  |
*PublicApi* | [**flagCommentPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#flagCommentPublic) | **POST** /flag-comment/{commentId} |  |
*PublicApi* | [**getCommentText**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentText) | **GET** /comments/{tenantId}/{commentId}/text |  |
*PublicApi* | [**getCommentVoteUserNames**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentVoteUserNames) | **GET** /comments/{tenantId}/{commentId}/votes |  |
*PublicApi* | [**getCommentsForUser**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentsForUser) | **GET** /comments-for-user |  |
*PublicApi* | [**getCommentsPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getCommentsPublic) | **GET** /comments/{tenantId} |  req tenantId urlId |
*PublicApi* | [**getEventLog**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getEventLog) | **GET** /event-log/{tenantId} |  req tenantId urlId userIdWS |
*PublicApi* | [**getFeedPostsPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getFeedPostsPublic) | **GET** /feed-posts/{tenantId} |  req tenantId afterId |
*PublicApi* | [**getFeedPostsStats**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getFeedPostsStats) | **GET** /feed-posts/{tenantId}/stats |  |
*PublicApi* | [**getGifLarge**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getGifLarge) | **GET** /gifs/get-large/{tenantId} |  |
*PublicApi* | [**getGifsSearch**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getGifsSearch) | **GET** /gifs/search/{tenantId} |  |
*PublicApi* | [**getGifsTrending**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getGifsTrending) | **GET** /gifs/trending/{tenantId} |  |
*PublicApi* | [**getGlobalEventLog**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getGlobalEventLog) | **GET** /event-log/global/{tenantId} |  req tenantId urlId userIdWS |
*PublicApi* | [**getOfflineUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getOfflineUsers) | **GET** /pages/{tenantId}/users/offline | Колишні коментатори на сторінці, які НЕ перебувають онлайн. Сортовано за displayName. Використовуйте це після вичерпання /users/online для відображення розділу \"Members\". Пагінація курсором за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName} від afterName вперед за допомогою $gt, без вартості $skip. |
*PublicApi* | [**getOnlineUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getOnlineUsers) | **GET** /pages/{tenantId}/users/online | Поточні онлайн‑переглядачі сторінки: люди, чиї веб‑socket сесії підписані на сторінку в даний момент. Повертає anonCount + totalCount (підписники всього приміщення, включаючи анонімних переглядачів, яких ми не перераховуємо). |
*PublicApi* | [**getPagesPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getPagesPublic) | **GET** /pages/{tenantId} | Список сторінок для орендаря. Використовується десктоп‑клієнтом FChat для заповнення списку кімнат. Потрібно, щоб `enableFChat` був true у розв’язаній кастомній конфігурації для кожної сторінки. Сторінки, які вимагають SSO, фільтруються відповідно до групового доступу запитуючого користувача. |
*PublicApi* | [**getTranslations**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getTranslations) | **GET** /translations/{namespace}/{component} |  |
*PublicApi* | [**getUserNotificationCount**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserNotificationCount) | **GET** /user-notifications/get-count |  |
*PublicApi* | [**getUserNotifications**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserNotifications) | **GET** /user-notifications |  |
*PublicApi* | [**getUserPresenceStatuses**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserPresenceStatuses) | **GET** /user-presence-status |  |
*PublicApi* | [**getUserReactsPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUserReactsPublic) | **GET** /feed-posts/{tenantId}/user-reacts |  |
*PublicApi* | [**getUsersInfo**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getUsersInfo) | **GET** /pages/{tenantId}/users/info | Bulk user info for a tenant. Given userIds, return display info from User / SSOUser. Used by the comment widget to enrich users that just appeared via a presence event. No page context: privacy is enforced uniformly (private profiles are masked). |
*PublicApi* | [**getV1PageLikes**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getV1PageLikes) | **GET** /page-reacts/v1/likes/{tenantId} |  |
*PublicApi* | [**getV2PageReactUsers**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getV2PageReactUsers) | **GET** /page-reacts/v2/{tenantId}/list |  |
*PublicApi* | [**getV2PageReacts**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#getV2PageReacts) | **GET** /page-reacts/v2/{tenantId} |  |
*PublicApi* | [**lockComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#lockComment) | **POST** /comments/{tenantId}/{commentId}/lock |  |
*PublicApi* | [**logoutPublic**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#logoutPublic) | **PUT** /auth/logout |  |
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
*PublicApi* | [**updateUserNotificationCommentSubscriptionStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateUserNotificationCommentSubscriptionStatus) | **POST** /user-notifications/{notificationId}/mark-opted/{optedInOrOut} | Увімкнути або вимкнути сповіщення для конкретного коментаря. |
*PublicApi* | [**updateUserNotificationPageSubscriptionStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateUserNotificationPageSubscriptionStatus) | **POST** /user-notifications/set-subscription-state/{subscribedOrUnsubscribed} | Увімкнути або вимкнути сповіщення для сторінки. Коли користувачі підписані на сторінку, сповіщення створюються для нових кореневих коментарів, і також |
*PublicApi* | [**updateUserNotificationStatus**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#updateUserNotificationStatus) | **POST** /user-notifications/{notificationId}/mark/{newStatus} |  |
*PublicApi* | [**uploadImage**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#uploadImage) | **POST** /upload-image/{tenantId} | Завантажити та змінити розмір зображення |
*PublicApi* | [**voteComment**](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Apis/PublicApi.md#voteComment) | **POST** /comments/{tenantId}/{commentId}/vote |  |


<a name="documentation-for-models"></a>
## Документація моделей

 - [APIAuditLog](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIAuditLog.md)
 - [APIBanUserChangeLog](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIBanUserChangeLog.md)
 - [APIBanUserChangedValues](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIBanUserChangedValues.md)
 - [APIBannedUser](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIBannedUser.md)
 - [APIBannedUserWithMultiMatchInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIBannedUserWithMultiMatchInfo.md)
 - [APIComment](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIComment.md)
 - [APICommentBase](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APICommentBase.md)
 - [APICommentBase_meta](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APICommentBase_meta.md)
 - [APICommentCommonBannedUser](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APICommentCommonBannedUser.md)
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
 - [APIModerateGetUserBanPreferencesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIModerateGetUserBanPreferencesResponse.md)
 - [APIModerateUserBanPreferences](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIModerateUserBanPreferences.md)
 - [APIPage](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIPage.md)
 - [APISSOUser](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APISSOUser.md)
 - [APISaveCommentResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APISaveCommentResponse.md)
 - [APIStatus](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIStatus.md)
 - [APITenant](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITenant.md)
 - [APITenantDailyUsage](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITenantDailyUsage.md)
 - [APITicket](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITicket.md)
 - [APITicketDetail](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITicketDetail.md)
 - [APITicketFile](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APITicketFile.md)
 - [APIUserSubscription](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/APIUserSubscription.md)
 - [AddDomainConfigParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddDomainConfigParams.md)
 - [AddDomainConfigResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddDomainConfigResponse.md)
 - [AddDomainConfigResponse_anyOf](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddDomainConfigResponse_anyOf.md)
 - [AddPageAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddPageAPIResponse.md)
 - [AddSSOUserAPIResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AddSSOUserAPIResponse.md)
 - [AdjustCommentVotesParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AdjustCommentVotesParams.md)
 - [AdjustVotesResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AdjustVotesResponse.md)
 - [AggregateQuestionResultsResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregateQuestionResultsResponse.md)
 - [AggregateResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregateResponse.md)
 - [AggregateTimeBucket](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregateTimeBucket.md)
 - [AggregationAPIError](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationAPIError.md)
 - [AggregationItem](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationItem.md)
 - [AggregationOpType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationOpType.md)
 - [AggregationOperation](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationOperation.md)
 - [AggregationRequest](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationRequest.md)
 - [AggregationRequest_sort](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationRequest_sort.md)
 - [AggregationResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationResponse.md)
 - [AggregationResponse_stats](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationResponse_stats.md)
 - [AggregationValue](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AggregationValue.md)
 - [AwardUserBadgeResponse](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/AwardUserBadgeResponse.md)
 - [BanUserFromCommentResult](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BanUserFromCommentResult.md)
 - [BanUserUndoParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BanUserUndoParams.md)
 - [BannedUserMatch](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BannedUserMatch.md)
 - [BannedUserMatchType](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BannedUserMatchType.md)
 - [BannedUserMatch_matchedOnValue](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BannedUserMatch_matchedOnValue.md)
 - [BillingInfo](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BillingInfo.md)
 - [BlockFromCommentParams](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BlockFromCommentParams.md)
 - [BlockSuccess](https://github.com/FastComments/fastcomments-cpp/blob/master/docs/Models/BlockSuccess.md)
 - [BuildModerationFilterParams](https://github.com