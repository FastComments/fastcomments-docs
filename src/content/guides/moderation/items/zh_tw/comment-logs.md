FastComments 會自動追蹤每則評論的詳細事件，以提供對審核決策與系統操作的透明度。這些日誌可幫助您了解評論為何被批准、被標記為垃圾郵件或其狀態為何變更。

## 存取評論日誌

要查看特定評論的日誌：

1. 前往您 FastComments 儀表板中的 **審核評論** 頁面
2. 找到您想檢查的評論
3. 在該評論的操作列中點選 **檢視日誌** 按鈕（時鐘圖示）
4. 會出現一個對話框，顯示該評論的完整事件歷史

每個日誌條目會顯示：
- **When** - 事件的時間戳記
- **Who** - 觸發該事件的使用者或系統（如適用）
- **What** - 動作或事件類型
- **Details** - 額外的背景資訊，例如前/後值、引擎名稱或相關資料

## 評論日誌事件

每則評論會在其生命週期中維護事件日誌。以下是會被追蹤的事件類型：

### Anonymization Events
- **Anonymized** - 評論內容已被清除且使用者標記為已刪除
- **RestoredFromAnonymized** - 評論已從匿名狀態還原

### Approval Events
- **ApprovedDueToPastComment** - 評論因該使用者先前有已獲批准的評論而被批准（包含對先前評論的參考）
- **ApprovedIsAdmin** - 評論因使用者為管理員而被批准
- **NotApprovedRequiresApproval** - 評論需要手動審核
- **NotApprovedLowTrustFactor** - 因使用者信任因子低而未被批准（包含該信任因子數值）

### Profile Comment Approval Events

These events apply specifically to comments on user profiles:

- **ApprovedProfileAutoApproveAll** - 個人檔案評論自動批准，因為個人檔案擁有者已啟用所有評論自動批准
- **ApprovedProfileTrusted** - 個人檔案評論被批准，因為發言者為可信使用者（包含建立信任的評論參考）
- **NotApprovedProfileManualApproveAll** - 個人檔案評論需要手動批准，因為個人檔案擁有者已啟用手動批准
- **NotApprovedProfileNotTrusted** - 個人檔案評論未被批准，因為發言者不受信任
- **NotApprovedProfileNewUser** - 個人檔案評論未被批准，因為發言者為新使用者

### Spam Detection Events
- **IsSpam** - 評論被偵測引擎標記為垃圾郵件（包含作出判定的引擎）
- **IsSpamDueToBadWords** - 因不當詞彙過濾而被標記為垃圾郵件
- **IsSpamFromLLM** - 由 AI/LLM 引擎標記為垃圾郵件（包含引擎名稱、回應與代幣數）
- **IsSpamRepeatComment** - 因重複性而被標記為垃圾郵件（包含偵測到的引擎）
- **NotSpamIsOnlyImage** - 僅包含圖片，因此未被標記為垃圾郵件
- **NotSpamIsOnlyReacts** - 僅包含表情反應，因此未被標記為垃圾郵件
- **NotSpamNoLinkOrMention** - 因無可疑連結或提及而未被標記為垃圾郵件
- **NotSpamPerfectTrustFactor** - 因使用者信任度高而未被標記為垃圾郵件
- **NotSpamTooShort** - 評論過短，無法分析，因此未被標記為垃圾郵件
- **NotSpamSkipped** - 跳過垃圾郵件檢查
- **NotSpamFromEngine** - 由偵測引擎判定為非垃圾郵件（包含引擎名稱與信任因子）

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - 髒話過濾檢查發生錯誤
- **BadWordsFoundBadPhrase** - 髒話過濾偵測到不當片語（包含該片語）
- **BadWordsFoundBadWord** - 髒話過濾偵測到不當單字（包含該單字）
- **BadWordsNoDefinitionForLocale** - 該語言無髒話定義（包含語系）

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 評論需要驗證但使用者不在已驗證的工作階段中
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 評論需要驗證但使用者尚未通過驗證
- **InVerifiedSession** - 發表評論的使用者處於已驗證的工作階段
- **SentVerificationEmailNoSession** - 驗證郵件已發送給未驗證的使用者
- **SentWelcomeEmail** - 歡迎郵件已發送給新使用者

### Trust and Security Events
- **TrustFactorChanged** - 使用者信任因子已變更（包含變更前後數值）
- **SpamFilterDisabledBecauseAdmin** - 對管理員使用者繞過垃圾郵件過濾
- **TenantSpamFilterDisabled** - 整個租戶已停用垃圾郵件過濾
- **RepeatCommentCheckIgnored** - 重複評論檢查被忽略（包含原因）
- **UserIsAdmin** - 使用者被識別為管理員
- **UserIsAdminParentTenant** - 使用者被識別為上層租戶的管理員
- **UserIsAdminViaSSO** - 使用者透過 SSO 被識別為管理員
- **UserIsMod** - 使用者被識別為版主

### Comment Status Changes

Status change events include before and after values, plus the user who made the change:

- **ExpireStatusChanged** - 評論到期狀態被修改
- **ReviewStatusChanged** - 評論審核狀態被變更
- **SpamStatusChanged** - 評論垃圾郵件狀態已更新
- **ApproveStatusChanged** - 評論批准狀態被變更
- **TextChanged** - 評論文字內容已編輯（包含編輯前後文字）
- **VotesChanged** - 評論投票數已更新（包含詳細投票分解）
- **Flagged** - 評論被使用者檢舉
- **UnFlagged** - 評論的檢舉已被移除

### Moderation Actions
- **Pinned** - 評論被版主釘選（包含釘選者）
- **UnPinned** - 評論被版主取消釘選（包含取消者）

### Notification Events
- **CreatedNotifications** - 已為評論建立通知（包含通知數量）
- **NotificationCreateFailure** - 建立通知失敗
- **BadgeAwarded** - 因評論授予使用者徽章（包含徽章名稱）

### Publishing Events
- **PublishedLive** - 評論已發佈給即時訂閱者（包含訂閱者人數）

### Integration Events
- **WebhookSynced** - 評論已透過 webhook 同步

### Spam Rule Events
- **SpamRuleMatch** - 評論符合自訂垃圾郵件規則（包含規則詳細）

### Localization Events
- **LocaleDetectedFromText** - 從評論文字自動偵測語言與語系（包含偵測到的語言與語系）

## 評論日誌的使用案例

評論日誌會自動產生並與每則評論一併儲存。它們提供了有價值的洞察，適用於：

- **理解審核決策** - 明確查看評論為何被批准、等待審核或被標記為垃圾郵件
- **除錯批准/垃圾郵件問題** - 在評論行為異常時追蹤決策邏輯
- **追蹤使用者行為模式** - 監控信任因子變化與驗證狀態
- **稽核管理者操作** - 檢視版主對特定評論所採取的操作
- **調查垃圾郵件過濾效能** - 檢視哪些偵測引擎能攔截垃圾郵件、哪些未能
- **整合故障排除** - 驗證 webhook 同步與通知傳遞

這些日誌有助於維持審核流程的透明度，並協助微調您的評論系統行為。