FastComments 自动跟踪每条评论的详细事件，以提供对审核决策和系统操作的透明度。这些日志帮助您了解为何评论被批准、被标记为垃圾信息或其状态被更改。

## 访问评论日志

要查看特定评论的日志：

1. 在您的 FastComments 仪表板中进入 **Moderate Comments** 页面  
2. 找到您想要检查的评论  
3. 点击评论操作栏中的 **View Logs** 按钮（时钟图标）  
4. 将弹出一个对话框，显示该评论的完整事件历史  

每条日志条目显示：

- **When** - 事件的时间戳  
- **Who** - 触发事件的用户或系统（如适用）  
- **What** - 动作或事件的类型  
- **Details** - 其他上下文信息，例如前后值、引擎名称或相关数据  

## 评论日志事件

每条评论在其生命周期中会维护一系列事件日志。以下是被跟踪的事件类型：

### 匿名化事件
- **Anonymized** - 评论内容被清除，用户标记为已删除  
- **RestoredFromAnonymized** - 评论从匿名化状态恢复  

### 审批事件
- **ApprovedDueToPastComment** - 因用户之前有已批准的评论而批准（包括对过去评论的引用）  
- **ApprovedIsAdmin** - 因用户是管理员而批准  
- **NotApprovedRequiresApproval** - 评论需要人工审批  
- **NotApprovedLowTrustFactor** - 因用户信任因子低而未批准（包括信任因子值）  

### 个人资料评论审批事件

这些事件专用于用户个人资料上的评论：

- **ApprovedProfileAutoApproveAll** - 因个人资料所有者启用了所有评论的自动批准而自动批准  
- **ApprovedProfileTrusted** - 因评论者被信任而批准（包括建立信任的评论引用）  
- **NotApprovedProfileManualApproveAll** - 因个人资料所有者启用了手动批准而需要人工审批  
- **NotApprovedProfileNotTrusted** - 因评论者未被信任而未批准  
- **NotApprovedProfileNewUser** - 因评论者是新用户而未批准  

### 垃圾信息检测事件
- **IsSpam** - 被检测引擎标记为垃圾信息（包括作出决定的引擎）  
- **IsSpamDueToBadWords** - 因不当词汇过滤器而标记为垃圾信息  
- **IsSpamFromLLM** - 被 AI/LLM 引擎标记为垃圾信息（包括引擎名称、响应和令牌计数）  
- **IsSpamRepeatComment** - 因重复性而标记为垃圾信息（包括检测到的引擎）  
- **NotSpamIsOnlyImage** - 因仅包含图片而未标记为垃圾信息  
- **NotSpamIsOnlyReacts** - 因仅包含表情而未标记为垃圾信息  
- **NotSpamNoLinkOrMention** - 因没有可疑链接或提及而未标记为垃圾信息  
- **NotSpamPerfectTrustFactor** - 因用户信任度高而未标记为垃圾信息  
- **NotSpamTooShort** - 因过短无法分析而未标记为垃圾信息  
- **NotSpamSkipped** - 垃圾信息检查被跳过  
- **NotSpamFromEngine** - 被检测引擎判定为非垃圾信息（包括引擎名称和信任因子）  

### 不当词汇/脏话事件
- **BadWordsCheckFailed** - 脏话过滤检查时出现错误  
- **BadWordsFoundBadPhrase** - 脏话过滤检测到不当短语（包括该短语）  
- **BadWordsFoundBadWord** - 脏话过滤检测到不当词汇（包括该词）  
- **BadWordsNoDefinitionForLocale** - 评论语言缺少脏话定义（包括语言地区）  

### 用户验证事件
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 评论需要验证，但用户不在已验证会话中  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 评论需要验证，但用户尚未完成验证  
- **InVerifiedSession** - 发布评论的用户处于已验证会话中  
- **SentVerificationEmailNoSession** - 向未验证用户发送验证邮件  
- **SentWelcomeEmail** - 向新用户发送欢迎邮件  

### 信任与安全事件
- **TrustFactorChanged** - 用户的信任因子被修改（包括前后值）  
- **SpamFilterDisabledBecauseAdmin** - 因管理员身份绕过垃圾信息过滤  
- **TenantSpamFilterDisabled** - 为整个租户禁用垃圾信息过滤  
- **RepeatCommentCheckIgnored** - 重复评论检查被绕过（包括原因）  
- **UserIsAdmin** - 用户被识别为管理员  
- **UserIsAdminParentTenant** - 用户被识别为父租户管理员  
- **UserIsAdminViaSSO** - 用户通过 SSO 被识别为管理员  
- **UserIsMod** - 用户被识别为版主  

### 评论状态变更

状态变更事件包括前后值以及执行变更的用户：

- **ExpireStatusChanged** - 评论的过期状态被修改  
- **ReviewStatusChanged** - 评论的审查状态被更改  
- **SpamStatusChanged** - 评论的垃圾信息状态被更新  
- **ApproveStatusChanged** - 评论的批准状态被更改  
- **TextChanged** - 评论文本内容被编辑（包括前后文本）  
- **VotesChanged** - 评论投票数被更新（包括详细投票分布）  
- **Flagged** - 评论被用户标记  
- **UnFlagged** - 评论的标记被移除  

### 审核操作
- **Pinned** - 评论被版主置顶（包括置顶者）  
- **UnPinned** - 评论被版主取消置顶（包括取消置顶者）  

### 通知事件
- **CreatedNotifications** - 为评论创建了通知（包括通知数量）  
- **NotificationCreateFailure** - 创建通知失败  
- **BadgeAwarded** - 为评论授予用户徽章（包括徽章名称）  

### 提及和回复通知事件

这些事件指明了将收到邮件或通知的人员。若未发送，Details 列会说明原因。

- **MentionEmailSent** - 被提及的用户收到了邮件  
- **MentionEmailSkipped** - 被提及的用户未收到邮件（包括原因）  
- **MentionHeldForApproval** - 提及邮件在评论批准前处于等待状态  
- **MentionNotificationCreated** - 被提及的用户收到了站内通知  
- **MentionNotificationSkipped** - 被提及的用户未收到站内通知（包括原因）  
- **ReplyEmailSent** - 被回复的评论作者收到了关于此回复的邮件  
- **ReplyEmailSkipped** - 被回复的评论作者未收到邮件（包括原因）  
- **ReplyNotificationSkipped** - 被回复的评论作者未收到站内通知（包括原因）  

未发送邮件或通知时显示的原因：

- 用户已不存在，或没有电子邮件地址  
- 用户关闭了电子邮件通知，或关闭了该主题的通知  
- 两位用户中有一方屏蔽了另一方  
- 用户不在任何相同的 SSO 组中  
- 用户的电子邮件地址因退回或垃圾邮件投诉被列入抑制列表（参见 [Email Suppression Management](/guide-notifications.html#email-suppression-management)）  
- 用户的电子邮件地址为 example.com，无法接收邮件  
- 评论被标记为垃圾信息、已删除或在 7 天内未获批准  
- 被回复的评论是匿名发布的  
- 用户回复了自己的评论  
- 用户在回复中被提及，因此收到的是提及邮件而非回复邮件  
- 用户已经收到该评论的回复通知  
- 发送失败 5 次  

如果投递失败或达到发送限制，邮件会进入重试队列，日志条目会相应记录。

### 发布事件
- **PublishedLive** - 评论已发布给实时订阅者（包括订阅者数量）  

### 集成事件
- **WebhookSynced** - 评论通过 webhook 同步  

### 垃圾信息规则事件
- **SpamRuleMatch** - 评论匹配了自定义垃圾信息规则（包括规则细节）  

### 本地化事件
- **LocaleDetectedFromText** - 根据评论文本自动检测到语言地区（包括检测到的语言和地区）  

## 评论日志的使用场景

评论日志会自动生成并随每条评论一起存储。它们为以下方面提供有价值的洞察：

- **了解审核决策** - 精确查看评论为何被批准、被审查或被标记为垃圾信息  
- **调试批准/垃圾信息问题** - 当评论行为异常时，追踪决策逻辑  
- **跟踪用户行为模式** - 监控信任因子变化和验证状态  
- **审计版主操作** - 回顾版主对特定评论执行的操作  
- **调查垃圾信息过滤效果** - 查看哪些检测引擎捕获了垃圾信息，哪些未捕获  
- **排查集成问题** - 验证 webhook 同步和通知投递  

这些日志帮助在审核过程中保持透明，并协助微调您的评论系统行为。