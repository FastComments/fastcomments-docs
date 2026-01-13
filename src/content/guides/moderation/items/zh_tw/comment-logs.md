FastComments 會自動追蹤每則評論的詳細事件，以提供對審核決定與系統動作的透明度。這些日誌能幫助您了解為什麼某則評論會被批准、被標為垃圾郵件，或其狀態被變更。

您可以在 審核評論 儀表板 中選取特定評論來查看該評論的日誌。

## Comment Log Events

Each comment maintains a log of events that occur during its lifecycle. Below are the types of events that are tracked:

### Anonymization Events
- **Anonymized** - 評論內容已被清除且使用者被標記為已刪除

### Approval Events
- **ApprovedDueToPastComment** - 因使用者先前有已獲批准的評論而獲批准
- **ApprovedIsAdmin** - 因使用者為管理員而獲批准
- **NotApprovedRequiresApproval** - 評論需要手動審核

### Spam Detection Events
- **IsSpam** - 評論被檢測引擎標記為垃圾郵件
- **IsSpamDueToBadWords** - 因髒話過濾而被標記為垃圾郵件
- **IsSpamFromLLM** - 由 AI/LLM 引擎判定為垃圾郵件
- **IsSpamRepeatComment** - 因為重複性而被標記為垃圾郵件
- **NotSpamIsOnlyImage** - 因只包含圖片而未被標記為垃圾郵件
- **NotSpamIsOnlyReacts** - 因只包含回應（反應）而未被標記為垃圾郵件
- **NotSpamNoLinkOrMention** - 因沒有可疑的連結或提及而未被標記為垃圾郵件
- **NotSpamPerfectTrustFactor** - 因使用者信任度高而未被標記為垃圾郵件
- **NotSpamTooShort** - 因過短無法分析而未被標記為垃圾郵件
- **NotSpamSkipped** - 已跳過垃圾郵件檢查
- **NotSpamFromEngine** - 檢測引擎判定非垃圾郵件

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - 髒話過濾檢查發生錯誤
- **BadWordsFoundBadPhrase** - 髒話過濾偵測到不當片語
- **BadWordsFoundBadWord** - 髒話過濾偵測到不當詞彙
- **BadWordsNoDefinitionForLocale** - 該評論語言沒有可用的髒話定義

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 評論需要驗證但使用者不在已驗證的會話中
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 評論需要驗證但使用者尚未完成驗證
- **InVerifiedSession** - 發表評論的使用者處於已驗證會話中
- **SentVerificationEmailNoSession** - 已向未驗證的使用者發送驗證電子郵件
- **SentWelcomeEmail** - 已向新使用者發送歡迎電子郵件

### Trust and Security Events
- **TrustFactorChanged** - 使用者的信任因子已被修改
- **SpamFilterDisabledBecauseAdmin** - 對管理員使用者繞過垃圾郵件過濾
- **TenantSpamFilterDisabled** - 已停用整個租戶的垃圾郵件過濾
- **RepeatCommentCheckIgnored** - 已忽略重複評論檢查
- **UserIsAdmin** - 使用者被識別為管理員
- **UserIsAdminParentTenant** - 使用者被識別為父級租戶管理員
- **UserIsAdminViaSSO** - 使用者透過 SSO 被識別為管理員
- **UserIsMod** - 使用者被識別為版主

### Comment Status Changes
- **ExpireStatusChanged** - 評論到期狀態已被修改
- **ReviewStatusChanged** - 評論的審核狀態已變更
- **SpamStatusChanged** - 評論的垃圾郵件狀態已更新
- **ApproveStatusChanged** - 評論的批准狀態已變更
- **TextChanged** - 評論文字內容已被編輯
- **VotesChanged** - 評論的投票數已更新
- **Flagged** - 評論被使用者檢舉
- **UnFlagged** - 評論的檢舉已被移除

### Moderation Actions
- **Pinned** - 評論被版主置頂
- **UnPinned** - 評論被版主取消置頂
- **RestoredFromAnonymized** - 評論已從匿名狀態還原

### Notification Events
- **CreatedNotifications** - 已為評論建立通知
- **NotificationCreateFailure** - 建立通知失敗
- **BadgeAwarded** - 使用者因評論獲得徽章

### Publishing Events
- **PublishedLive** - 評論已發佈給即時訂閱者

### Integration Events
- **WebhookSynced** - 評論已透過 webhook 同步

### Spam Rule Events
- **SpamRuleMatch** - 評論符合自訂垃圾郵件規則

## Accessing Comment Logs

Comment logs are automatically generated and stored with each comment. They provide valuable insights for:

- Understanding moderation decisions
- Debugging approval/spam issues
- Tracking user behavior patterns
- Auditing system actions

These logs help maintain transparency in the moderation process and assist in fine-tuning your comment system's behavior.