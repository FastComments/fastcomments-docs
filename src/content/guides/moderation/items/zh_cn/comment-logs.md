FastComments 会自动跟踪每条评论的详细事件，以便对审核决定和系统操作提供透明度。这些日志可帮助您了解评论为何被批准、被标记为垃圾信息，或其状态为何被更改。

您可以在审核评论仪表板中选择特定评论以查看该评论的日志。

## 评论日志事件

每条评论都会维护在其生命周期中发生的事件日志。以下是被跟踪的事件类型：

### 匿名化事件
- **Anonymized** - 评论内容已被清除，用户被标记为已删除

### 批准事件
- **ApprovedDueToPastComment** - 因为该用户之前的评论被批准，评论因此被批准
- **ApprovedIsAdmin** - 因为用户是管理员，评论被批准
- **NotApprovedRequiresApproval** - 评论需要人工审核

### 垃圾检测事件
- **IsSpam** - 评论被检测引擎标记为垃圾信息
- **IsSpamDueToBadWords** - 因为脏话过滤，评论被标记为垃圾信息
- **IsSpamFromLLM** - 被 AI/LLM 引擎标记为垃圾信息
- **IsSpamRepeatComment** - 因为重复内容，评论被标记为垃圾信息
- **NotSpamIsOnlyImage** - 评论仅包含图片，因此未被标记为垃圾信息
- **NotSpamIsOnlyReacts** - 评论仅包含表情/反应，因此未被标记为垃圾信息
- **NotSpamNoLinkOrMention** - 评论没有可疑链接或提及，因此未被标记为垃圾信息
- **NotSpamPerfectTrustFactor** - 由于用户信任度高，评论未被标记为垃圾信息
- **NotSpamTooShort** - 评论过短，无法分析，因此未被标记为垃圾信息
- **NotSpamSkipped** - 已跳过垃圾检测
- **NotSpamFromEngine** - 检测引擎判断评论非垃圾信息

### 不当用语/脏词事件
- **BadWordsCheckFailed** - 脏词过滤检查时发生错误
- **BadWordsFoundBadPhrase** - 脏词过滤检测到不当短语
- **BadWordsFoundBadWord** - 脏词过滤检测到不当词语
- **BadWordsNoDefinitionForLocale** - 未为评论语言提供脏词定义

### 用户验证事件
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 评论需要验证，但用户不在已验证会话中
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 评论需要验证，但用户尚未验证
- **InVerifiedSession** - 发布评论的用户处于已验证会话中
- **SentVerificationEmailNoSession** - 已向未验证用户发送验证电子邮件
- **SentWelcomeEmail** - 已向新用户发送欢迎邮件

### 信任与安全事件
- **TrustFactorChanged** - 用户的信任因子已被修改
- **SpamFilterDisabledBecauseAdmin** - 管理员用户绕过了垃圾过滤
- **TenantSpamFilterDisabled** - 整个租户已禁用垃圾过滤
- **RepeatCommentCheckIgnored** - 重复评论检查被忽略
- **UserIsAdmin** - 用户被识别为管理员
- **UserIsAdminParentTenant** - 用户被识别为父租户管理员
- **UserIsAdminViaSSO** - 用户通过 SSO 被识别为管理员
- **UserIsMod** - 用户被识别为版主

### 评论状态更改
- **ExpireStatusChanged** - 评论过期状态已被修改
- **ReviewStatusChanged** - 评论审核状态已更改
- **SpamStatusChanged** - 评论垃圾状态已更新
- **ApproveStatusChanged** - 评论批准状态已更改
- **TextChanged** - 评论文本内容已被编辑
- **VotesChanged** - 评论投票计数已更新
- **Flagged** - 评论被用户标记
- **UnFlagged** - 评论的标记已被移除

### 审核操作
- **Pinned** - 评论被版主置顶
- **UnPinned** - 评论被版主取消置顶
- **RestoredFromAnonymized** - 评论已从匿名化状态恢复

### 通知事件
- **CreatedNotifications** - 已为评论创建通知
- **NotificationCreateFailure** - 创建通知失败
- **BadgeAwarded** - 因评论向用户授予了徽章

### 发布事件
- **PublishedLive** - 评论已发布给实时订阅用户

### 集成事件
- **WebhookSynced** - 评论已通过 webhook 同步

### 垃圾规则事件
- **SpamRuleMatch** - 评论匹配了自定义的垃圾规则

## 访问评论日志

评论日志会自动生成并与每条评论一并存储。它们提供了有价值的见解，用于：

- 理解审核决定
- 调试批准/垃圾信息问题
- 跟踪用户行为模式
- 审计系统操作

这些日志有助于在审核过程中保持透明度，并协助微调评论系统的行为。