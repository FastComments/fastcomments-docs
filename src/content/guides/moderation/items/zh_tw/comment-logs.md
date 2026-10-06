FastComments 會自動追蹤每則評論的詳細事件，以提供審核決策與系統操作的透明度。這些日誌可協助您了解為何評論被批准、被標記為垃圾訊息，或其狀態被變更。

## 取得評論日誌

要檢視特定評論的日誌：

1. 前往 FastComments 控制台中的 **Moderate Comments**（審核評論）頁面
2. 找到您想要檢查的評論
3. 在評論的操作列中點擊 **View Logs**（檢視日誌）按鈕（時鐘圖示）
4. 會彈出對話框，顯示該評論的完整事件歷史

每筆日誌條目會顯示：

- **When** - 事件的時間戳記
- **Who** - 觸發事件的使用者或系統（若適用）
- **What** - 動作或事件的類型
- **Details** - 其他上下文資訊，例如前後值、引擎名稱或相關資料

## 評論日誌事件

每則評論都會保留其生命週期中發生的事件日誌。以下列出所追蹤的事件類型：

### 匿名化事件
- **Anonymized** - 評論內容被清除，且使用者被標記為已刪除
- **RestoredFromAnonymized** - 評論從匿名狀態中恢復

### 核准事件
- **ApprovedDueToPastComment** - 因使用者先前已核准過評論而核准此評論（包含對過去評論的參照）
- **ApprovedIsAdmin** - 因使用者是管理員而核准此評論
- **NotApprovedRequiresApproval** - 此評論需要手動核准
- **NotApprovedLowTrustFactor** - 因使用者信任因子過低而未核准此評論（包含信任因子值）

### 個人檔案評論核准事件
這些事件專門適用於使用者個人檔案上的評論：

- **ApprovedProfileAutoApproveAll** - 因個人檔案擁有者已啟用所有評論自動核准，故此個人檔案評論自動核准
- **ApprovedProfileTrusted** - 因評論者被信任而核准此個人檔案評論（包含建立信任的評論參照）
- **NotApprovedProfileManualApproveAll** - 因個人檔案擁有者已啟用手動核准，故此個人檔案評論需要手動核准
- **NotApprovedProfileNotTrusted** - 因評論者未被信任而未核准此個人檔案評論
- **NotApprovedProfileNewUser** - 因評論者是新使用者而未核准此個人檔案評論

### 垃圾訊息偵測事件
- **IsSpam** - 由偵測引擎標記為垃圾訊息（包含作出決策的引擎）
- **IsSpamDueToBadWords** - 因髒話過濾器而被標記為垃圾訊息
- **IsSpamFromLLM** - 由 AI/LLM 引擎標記為垃圾訊息（包含引擎名稱、回應與 token 數量）
- **IsSpamRepeatComment** - 因重複性而被標記為垃圾訊息（包含偵測此情況的引擎）
- **NotSpamIsOnlyImage** - 因僅包含圖片而未被標記為垃圾訊息
- **NotSpamIsOnlyReacts** - 因僅包含回應（reactions）而未被標記為垃圾訊息
- **NotSpamNoLinkOrMention** - 因沒有可疑連結或提及而未被標記為垃圾訊息
- **NotSpamPerfectTrustFactor** - 因使用者信任度高而未被標記為垃圾訊息
- **NotSpamTooShort** - 因過短無法分析而未被標記為垃圾訊息
- **NotSpamSkipped** - 已跳過垃圾訊息檢查
- **NotSpamFromEngine** - 由偵測引擎判定非垃圾訊息（包含引擎名稱與信任因子）

### 不當字詞/髒話事件
- **BadWordsCheckFailed** - 髒話過濾檢查時發生錯誤
- **BadWordsFoundBadPhrase** - 髒話過濾偵測到不當片語（包含該片語）
- **BadWordsFoundBadWord** - 髒話過濾偵測到不當字詞（包含該字詞）
- **BadWordsNoDefinitionForLocale** - 無該評論語言的髒話定義（包含語系）

### 使用者驗證事件
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 評論需要驗證，但使用者不在已驗證的會話中
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 評論需要驗證，但使用者尚未驗證
- **InVerifiedSession** - 發表評論的使用者處於已驗證的會話中
- **SentVerificationEmailNoSession** - 已向未驗證的使用者發送驗證電子郵件
- **SentWelcomeEmail** - 已向新使用者發送歡迎電子郵件

### 信任與安全事件
- **TrustFactorChanged** - 使用者的信任因子已變更（包含前後值）
- **SpamFilterDisabledBecauseAdmin** - 因管理員身分而繞過垃圾訊息過濾
- **TenantSpamFilterDisabled** - 為整個租戶停用垃圾訊息過濾
- **RepeatCommentCheckIgnored** - 已繞過重複評論檢查（包含原因）
- **UserIsAdmin** - 使用者被識別為管理員
- **UserIsAdminParentTenant** - 使用者被識別為父租戶管理員
- **UserIsAdminViaSSO** - 使用者透過 SSO 被識別為管理員
- **UserIsMod** - 使用者被識別為版主

### 評論狀態變更
狀態變更事件會包含前後值，以及執行變更的使用者：

- **ExpireStatusChanged** - 評論的過期狀態已變更
- **ReviewStatusChanged** - 評論的審核狀態已變更
- **SpamStatusChanged** - 評論的垃圾訊息狀態已更新
- **ApproveStatusChanged** - 評論的核准狀態已變更
- **TextChanged** - 評論文字內容已編輯（包含前後文字）
- **VotesChanged** - 評論的投票數已更新（包含詳細投票分布）
- **Flagged** - 評論被使用者標記
- **UnFlagged** - 評論的標記已移除

### 審核操作
- **Pinned** - 評論被版主置頂（包含置頂者）
- **UnPinned** - 評論被版主取消置頂（包含取消置頂者）

### 通知事件
- **CreatedNotifications** - 為評論建立了通知（包含通知數量）
- **NotificationCreateFailure** - 建立通知失敗
- **BadgeAwarded** - 使用者因評論獲得徽章（包含徽章名稱）

### 提及與回覆通知事件
這些事件會指明將收到電子郵件或通知的人。若未發送任何內容，Details 欄位會說明原因。

- **MentionEmailSent** - 被評論提及的使用者已收到電子郵件
- **MentionEmailSkipped** - 被提及的使用者未收到電子郵件（包含原因）
- **MentionHeldForApproval** - 提及郵件將等候評論核准後再發送
- **MentionNotificationCreated** - 被提及的使用者收到應用內通知
- **MentionNotificationSkipped** - 被提及的使用者未收到應用內通知（包含原因）
- **ReplyEmailSent** - 被回覆的評論作者已收到關於此回覆的電子郵件
- **ReplyEmailSkipped** - 被回覆的評論作者未收到電子郵件（包含原因）
- **ReplyNotificationSkipped** - 被回覆的評論作者未收到應用內通知（包含原因）

未發送電子郵件或通知時顯示的原因：

- 使用者已不存在，或沒有電子郵件地址
- 使用者關閉了電子郵件通知，或關閉了該討論串的通知
- 其中一位使用者已封鎖另一位
- 這兩位使用者不屬於任何相同的 SSO 群組
- 使用者的電子郵件地址因退信或垃圾郵件投訴而被列入抑制清單（請參閱 [Email Suppression Management](/guide-notifications.html#email-suppression-management)）
- 使用者的電子郵件地址為 example.com，無法接收郵件
- 該評論在 7 天內被標記為垃圾訊息、刪除或未核准
- 被回覆的評論是匿名發表的
- 使用者回覆了自己的評論
- 使用者在回覆中被提及，因此收到提及郵件而非回覆郵件
- 使用者已經收到該評論的回覆通知
- 發送失敗 5 次

如果投遞失敗或達到發送上限，電子郵件會排入佇列重試，且日誌條目會說明此情況。

### 發佈事件
- **PublishedLive** - 評論已發佈給即時訂閱者（包含訂閱者數量）

### 整合事件
- **WebhookSynced** - 評論已透過 webhook 同步

### 垃圾訊息規則事件
- **SpamRuleMatch** - 評論符合自訂垃圾訊息規則（包含規則細節）

### 本地化事件
- **LocaleDetectedFromText** - 從評論文字自動偵測到語言區域（包含偵測到的語言與區域）

## 評論日誌的使用情境

評論日誌會自動產生並與每則評論一起儲存。它們提供以下寶貴的見解：

- **Understanding moderation decisions** - 了解評論被批准、保留審核或標記為垃圾訊息的確切原因
- **Debugging approval/spam issues** - 當評論未如預期運作時，追蹤決策邏輯
- **Tracking user behavior patterns** - 監控信任因子變化與驗證狀態
- **Auditing moderator actions** - 檢視版主對特定評論所採取的操作
- **Investigating spam filter effectiveness** - 了解哪些偵測引擎能捕捉垃圾訊息，哪些則無法
- **Troubleshooting integrations** - 驗證 webhook 同步與通知傳遞

這些日誌有助於在審核過程中保持透明，並協助微調您的評論系統行為。