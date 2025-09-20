FastComments automatically tracks detailed events for each comment to provide transparency into moderation decisions and system actions. These logs help you understand why a comment was approved, flagged as spam, or had its status changed.

You can view comment logs for individual comments in the Moderate Comments dashboard by selecting a specific comment.

## Comment Log Events

Each comment maintains a log of events that occur during its lifecycle. Below are the types of events that are tracked:

### Anonymization Events
- **Anonymized** - Comment content was cleared and user marked as deleted

### Approval Events
- **ApprovedDueToPastComment** - Comment approved because user has previously approved comments
- **ApprovedIsAdmin** - Comment approved because user is an admin
- **NotApprovedRequiresApproval** - Comment requires manual approval

### Spam Detection Events
- **IsSpam** - Comment flagged as spam by detection engine
- **IsSpamDueToBadWords** - Comment flagged as spam due to profanity filter
- **IsSpamFromLLM** - Comment flagged as spam by AI/LLM engine
- **IsSpamRepeatComment** - Comment flagged as spam for being repetitive
- **NotSpamIsOnlyImage** - Comment not flagged as spam because it only contains images
- **NotSpamIsOnlyReacts** - Comment not flagged as spam because it only contains reactions
- **NotSpamNoLinkOrMention** - Comment not flagged as spam due to no suspicious links or mentions
- **NotSpamPerfectTrustFactor** - Comment not flagged as spam due to high user trust
- **NotSpamTooShort** - Comment not flagged as spam because it's too short to analyze
- **NotSpamSkipped** - Spam check was skipped
- **NotSpamFromEngine** - Comment determined not spam by detection engine

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Profanity filter check encountered an error
- **BadWordsFoundBadPhrase** - Profanity filter detected inappropriate phrase
- **BadWordsFoundBadWord** - Profanity filter detected inappropriate word
- **BadWordsNoDefinitionForLocale** - No profanity definitions available for comment language

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Comment requires verification but user not in verified session
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Comment requires verification but user not yet verified
- **InVerifiedSession** - User posting comment is in a verified session
- **SentVerificationEmailNoSession** - Verification email sent to unverified user
- **SentWelcomeEmail** - Welcome email sent to new user

### Trust and Security Events
- **TrustFactorChanged** - User's trust factor was modified
- **SpamFilterDisabledBecauseAdmin** - Spam filtering bypassed for admin user
- **TenantSpamFilterDisabled** - Spam filtering disabled for entire tenant
- **RepeatCommentCheckIgnored** - Repeat comment check was bypassed
- **UserIsAdmin** - User identified as admin
- **UserIsAdminParentTenant** - User identified as parent tenant admin
- **UserIsAdminViaSSO** - User identified as admin via SSO
- **UserIsMod** - User identified as moderator

### Comment Status Changes
- **ExpireStatusChanged** - Comment expiration status was modified
- **ReviewStatusChanged** - Comment review status was changed
- **SpamStatusChanged** - Comment spam status was updated
- **ApproveStatusChanged** - Comment approval status was changed
- **TextChanged** - Comment text content was edited
- **VotesChanged** - Comment vote counts were updated
- **Flagged** - Comment was flagged by users
- **UnFlagged** - Comment flags were removed

### Moderation Actions
- **Pinned** - Comment was pinned by moderator
- **UnPinned** - Comment was unpinned by moderator
- **RestoredFromAnonymized** - Comment was restored from anonymized state

### Notification Events
- **CreatedNotifications** - Notifications were created for comment
- **NotificationCreateFailure** - Failed to create notifications
- **BadgeAwarded** - User badge was awarded for comment

### Publishing Events
- **PublishedLive** - Comment was published to live subscribers

### Integration Events
- **WebhookSynced** - Comment was synchronized via webhook

### Spam Rule Events
- **SpamRuleMatch** - Comment matched a custom spam rule

## Accessing Comment Logs

Comment logs are automatically generated and stored with each comment. They provide valuable insights for:

- Understanding moderation decisions
- Debugging approval/spam issues
- Tracking user behavior patterns
- Auditing system actions

These logs help maintain transparency in the moderation process and assist in fine-tuning your comment system's behavior.