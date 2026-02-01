FastComments automatically tracks detailed events for each comment to provide transparency into moderation decisions and system actions. These logs help you understand why a comment was approved, flagged as spam, or had its status changed.

## Accessing Comment Logs

To view the logs for a specific comment:

1. Navigate to the **Moderate Comments** page in your FastComments dashboard
2. Find the comment you want to inspect
3. Click the **View Logs** button (clock icon) in the comment's action bar
4. A dialog will appear showing the complete history of events for that comment

Each log entry displays:
- **When** - The timestamp of the event
- **Who** - The user or system that triggered the event (when applicable)
- **What** - The type of action or event
- **Details** - Additional context such as before/after values, engine names, or related data

## Comment Log Events

Each comment maintains a log of events that occur during its lifecycle. Below are the types of events that are tracked:

### Anonymization Events
- **Anonymized** - Comment content was cleared and user marked as deleted
- **RestoredFromAnonymized** - Comment was restored from anonymized state

### Approval Events
- **ApprovedDueToPastComment** - Comment approved because user has previously approved comments (includes reference to the past comment)
- **ApprovedIsAdmin** - Comment approved because user is an admin
- **NotApprovedRequiresApproval** - Comment requires manual approval
- **NotApprovedLowTrustFactor** - Comment not approved due to low user trust factor (includes the trust factor value)

### Profile Comment Approval Events

These events apply specifically to comments on user profiles:

- **ApprovedProfileAutoApproveAll** - Profile comment auto-approved because the profile owner has enabled auto-approve for all comments
- **ApprovedProfileTrusted** - Profile comment approved because the commenter is trusted (includes reference to the comment that established trust)
- **NotApprovedProfileManualApproveAll** - Profile comment requires manual approval because the profile owner has enabled manual approval
- **NotApprovedProfileNotTrusted** - Profile comment not approved because the commenter is not trusted
- **NotApprovedProfileNewUser** - Profile comment not approved because the commenter is a new user

### Spam Detection Events
- **IsSpam** - Comment flagged as spam by detection engine (includes which engine made the decision)
- **IsSpamDueToBadWords** - Comment flagged as spam due to profanity filter
- **IsSpamFromLLM** - Comment flagged as spam by AI/LLM engine (includes engine name, response, and token count)
- **IsSpamRepeatComment** - Comment flagged as spam for being repetitive (includes which engine detected it)
- **NotSpamIsOnlyImage** - Comment not flagged as spam because it only contains images
- **NotSpamIsOnlyReacts** - Comment not flagged as spam because it only contains reactions
- **NotSpamNoLinkOrMention** - Comment not flagged as spam due to no suspicious links or mentions
- **NotSpamPerfectTrustFactor** - Comment not flagged as spam due to high user trust
- **NotSpamTooShort** - Comment not flagged as spam because it's too short to analyze
- **NotSpamSkipped** - Spam check was skipped
- **NotSpamFromEngine** - Comment determined not spam by detection engine (includes engine name and trust factor)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Profanity filter check encountered an error
- **BadWordsFoundBadPhrase** - Profanity filter detected inappropriate phrase (includes the phrase)
- **BadWordsFoundBadWord** - Profanity filter detected inappropriate word (includes the word)
- **BadWordsNoDefinitionForLocale** - No profanity definitions available for comment language (includes the locale)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Comment requires verification but user not in verified session
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Comment requires verification but user not yet verified
- **InVerifiedSession** - User posting comment is in a verified session
- **SentVerificationEmailNoSession** - Verification email sent to unverified user
- **SentWelcomeEmail** - Welcome email sent to new user

### Trust and Security Events
- **TrustFactorChanged** - User's trust factor was modified (includes before and after values)
- **SpamFilterDisabledBecauseAdmin** - Spam filtering bypassed for admin user
- **TenantSpamFilterDisabled** - Spam filtering disabled for entire tenant
- **RepeatCommentCheckIgnored** - Repeat comment check was bypassed (includes the reason)
- **UserIsAdmin** - User identified as admin
- **UserIsAdminParentTenant** - User identified as parent tenant admin
- **UserIsAdminViaSSO** - User identified as admin via SSO
- **UserIsMod** - User identified as moderator

### Comment Status Changes

Status change events include before and after values, plus the user who made the change:

- **ExpireStatusChanged** - Comment expiration status was modified
- **ReviewStatusChanged** - Comment review status was changed
- **SpamStatusChanged** - Comment spam status was updated
- **ApproveStatusChanged** - Comment approval status was changed
- **TextChanged** - Comment text content was edited (includes before and after text)
- **VotesChanged** - Comment vote counts were updated (includes detailed vote breakdown)
- **Flagged** - Comment was flagged by users
- **UnFlagged** - Comment flags were removed

### Moderation Actions
- **Pinned** - Comment was pinned by moderator (includes who pinned it)
- **UnPinned** - Comment was unpinned by moderator (includes who unpinned it)

### Notification Events
- **CreatedNotifications** - Notifications were created for comment (includes notification count)
- **NotificationCreateFailure** - Failed to create notifications
- **BadgeAwarded** - User badge was awarded for comment (includes badge name)

### Publishing Events
- **PublishedLive** - Comment was published to live subscribers (includes subscriber count)

### Integration Events
- **WebhookSynced** - Comment was synchronized via webhook

### Spam Rule Events
- **SpamRuleMatch** - Comment matched a custom spam rule (includes rule details)

### Localization Events
- **LocaleDetectedFromText** - Language locale was automatically detected from comment text (includes detected language and locale)

## Use Cases for Comment Logs

Comment logs are automatically generated and stored with each comment. They provide valuable insights for:

- **Understanding moderation decisions** - See exactly why a comment was approved, held for review, or marked as spam
- **Debugging approval/spam issues** - Trace through the decision logic when comments aren't behaving as expected
- **Tracking user behavior patterns** - Monitor trust factor changes and verification status
- **Auditing moderator actions** - Review what actions moderators have taken on specific comments
- **Investigating spam filter effectiveness** - See which detection engines are catching spam and which are not
- **Troubleshooting integrations** - Verify webhook syncs and notification delivery

These logs help maintain transparency in the moderation process and assist in fine-tuning your comment system's behavior.