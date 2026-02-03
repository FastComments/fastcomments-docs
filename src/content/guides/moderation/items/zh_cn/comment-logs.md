FastComments 会自动跟踪每条评论的详细事件，以便对审核决策和系统操作保持透明。这些日志可帮助您了解评论为何被批准、被标记为垃圾邮件或其状态为何发生更改。

## 访问评论日志

要查看特定评论的日志：

1. 导航到您 FastComments 仪表板中的 **审核评论** 页面
2. 找到您想要检查的评论
3. 单击该评论操作栏中的 **查看日志** 按钮（时钟图标）
4. 会弹出一个对话框，显示该评论的完整事件历史

每条日志条目显示：
- **何时** - 事件的时间戳
- **谁** - 触发该事件的用户或系统（如适用）
- **什么** - 操作或事件的类型
- **详细信息** - 附加上下文，例如更改前/后值、引擎名称或相关数据

## 评论日志事件

每条评论在其生命周期中都会维护一个事件日志。以下是跟踪的事件类型：

### 匿名化事件
- **Anonymized** - 评论内容已被清除，用户被标记为已删除
- **RestoredFromAnonymized** - 评论已从匿名状态恢复

### 审批事件
- **ApprovedDueToPastComment** - 评论被批准，因为该用户之前有被批准的评论（包含对过去评论的引用）
- **ApprovedIsAdmin** - 评论被批准，因为用户是管理员
- **NotApprovedRequiresApproval** - 评论需要人工审批
- **NotApprovedLowTrustFactor** - 评论未被批准，原因是用户信任度低（包含信任度值）

### 个人资料评论审批事件

这些事件专门适用于用户个人资料上的评论：

- **ApprovedProfileAutoApproveAll** - 个人资料评论被自动批准，因为个人资料所有者已启用对所有评论自动批准
- **ApprovedProfileTrusted** - 个人资料评论被批准，因为评论者是受信任的用户（包含建立信任的评论引用）
- **NotApprovedProfileManualApproveAll** - 个人资料评论需要人工审批，因为个人资料所有者已启用人工审核
- **NotApprovedProfileNotTrusted** - 个人资料评论未被批准，因为评论者不受信任
- **NotApprovedProfileNewUser** - 个人资料评论未被批准，因为评论者是新用户

### 垃圾邮件检测事件
- **IsSpam** - 评论被检测引擎标记为垃圾邮件（包含做出决定的引擎）
- **IsSpamDueToBadWords** - 评论因粗俗词汇过滤器被标记为垃圾邮件
- **IsSpamFromLLM** - 评论被 AI/LLM 引擎标记为垃圾邮件（包含引擎名称、响应和令牌数）
- **IsSpamRepeatComment** - 评论因重复性被标记为垃圾邮件（包含检测到它的引擎）
- **NotSpamIsOnlyImage** - 评论未被标记为垃圾邮件，因为它仅包含图片
- **NotSpamIsOnlyReacts** - 评论未被标记为垃圾邮件，因为它仅包含反应
- **NotSpamNoLinkOrMention** - 评论未被标记为垃圾邮件，因为没有可疑的链接或提及
- **NotSpamPerfectTrustFactor** - 评论未被标记为垃圾邮件，因为用户信任度很高
- **NotSpamTooShort** - 评论未被标记为垃圾邮件，因为它太短而无法分析
- **NotSpamSkipped** - 跳过了垃圾邮件检查
- **NotSpamFromEngine** - 评论被检测引擎判定为非垃圾邮件（包含引擎名称和信任度）

### 违禁词/粗俗事件
- **BadWordsCheckFailed** - 粗俗词过滤检查遇到错误
- **BadWordsFoundBadPhrase** - 粗俗词过滤检测到不当短语（包含该短语）
- **BadWordsFoundBadWord** - 粗俗词过滤检测到不当词汇（包含该词）
- **BadWordsNoDefinitionForLocale** - 没有可用于评论语言的粗俗词定义（包含区域设置）

### 用户验证事件
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 评论需要验证但用户不在已验证会话中
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 评论需要验证但用户尚未验证
- **InVerifiedSession** - 发布评论的用户处于已验证会话中
- **SentVerificationEmailNoSession** - 验证邮件已发送给未验证的用户
- **SentWelcomeEmail** - 欢迎邮件已发送给新用户

### 信任与安全事件
- **TrustFactorChanged** - 用户的信任度已被修改（包含修改前后值）
- **SpamFilterDisabledBecauseAdmin** - 管理员用户绕过了垃圾邮件过滤
- **TenantSpamFilterDisabled** - 整个租户已禁用垃圾邮件过滤
- **RepeatCommentCheckIgnored** - 重复评论检查被忽略（包含原因）
- **UserIsAdmin** - 用户被识别为管理员
- **UserIsAdminParentTenant** - 用户被识别为父租户管理员
- **UserIsAdminViaSSO** - 用户通过 SSO 被识别为管理员
- **UserIsMod** - 用户被识别为版主

### 评论状态更改

状态更改事件包含更改前后值，以及执行更改的用户：

- **ExpireStatusChanged** - 评论过期状态已被修改
- **ReviewStatusChanged** - 评论审核状态已更改
- **SpamStatusChanged** - 评论垃圾邮件状态已更新
- **ApproveStatusChanged** - 评论审批状态已更改
- **TextChanged** - 评论文本内容被编辑（包含编辑前后文本）
- **VotesChanged** - 评论投票数已更新（包含详细的投票细分）
- **Flagged** - 评论被用户标记
- **UnFlagged** - 评论标记已被移除

### 审核操作
- **Pinned** - 评论被版主置顶（包含置顶者）
- **UnPinned** - 评论被版主取消置顶（包含取消置顶者）

### 通知事件
- **CreatedNotifications** - 已为评论创建通知（包含通知数量）
- **NotificationCreateFailure** - 创建通知失败
- **BadgeAwarded** - 用户因评论获得徽章（包含徽章名称）

### 发布事件
- **PublishedLive** - 评论已发布给实时订阅者（包含订阅者数量）

### 集成事件
- **WebhookSynced** - 评论已通过 webhook 同步

### 垃圾规则事件
- **SpamRuleMatch** - 评论匹配自定义垃圾规则（包含规则详细信息）

### 本地化事件
- **LocaleDetectedFromText** - 从评论文本自动检测到语言区域（包含检测到的语言和区域设置）

## 评论日志的使用场景

评论日志会自动生成并与每条评论一起存储。它们为以下方面提供了宝贵的见解：

- **理解审核决策** - 精确查看评论为何被批准、被保留审核或被标记为垃圾邮件
- **调试审批/垃圾邮件问题** - 在评论行为异常时，追踪决策逻辑
- **跟踪用户行为模式** - 监控信任度变化和验证状态
- **审计版主操作** - 审查版主对特定评论所采取的操作
- **调查垃圾过滤器的有效性** - 查看哪些检测引擎在识别垃圾邮件，哪些没有
- **故障排查集成问题** - 验证 webhook 同步和通知发送

这些日志有助于在审核过程中保持透明，并帮助微调您的评论系统行为。