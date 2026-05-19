The FastComments LTI 1.3 集成遵循最小权限原则：它仅使用识别用户、将评论附加到正确的课程和资源以及应用基于角色的权限所需的启动声明。

本页其余部分映射了集成所使用的每个声明、不请求的每项 LTI Advantage 服务以及不收集的每类数据。安全与采购审查人员可以直接从下表中提取答案。

## 从 LMS 接收的数据元素

每次 LTI 1.3 启动都携带来自 LMS 的签名 JWT。FastComments 从该 JWT 中提取以下声明，并且不使用其他内容：

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | 在多次启动中一致地标识用户，使同一人解析为相同的 FastComments SSO 用户 | 是 | 是，作为稳定的内部 SSO ID 的一部分 |
| Display name | `name` | 显示在用户评论旁的署名 | 是（如果缺失则回退为 "LMS User"） | 是 |
| Email | `email` | 帐户匹配、通知、审核、支持通信 | 可选（集成在没有它的情况下仍然可以工作） | 提供时会存储 |
| Avatar URL | `picture` | 显示在用户的评论上 | 可选 | 仅存储 URL；FastComments 不会下载或重新托管该图像 |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | 确定用户是管理员、讲师（版主）还是学习者 | 是 | 在 SSO 会话上派生出 `isAdmin` / `isModerator` 标志 |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | 将评论线程关联到正确的 LMS 课程 | 是 | 是，作为解析后页面标识的一部分 |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | 将评论与课程内正确的活动或工具位置关联 | 如果存在则为必需 | 是，作为解析后页面标识的一部分 |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | 将启动路由到正确的 FastComments 租户配置 | 是 | 是，存储在 FastComments LTI 配置记录中 |

## 在注册时声明的声明和范围

在 LTI 1.3 动态注册期间，FastComments 以 `scope: ""`（无额外 OAuth 范围）注册自身，并仅声明以下 OpenID Connect 声明：

`iss`, `sub`, `name`, `email`, `picture`

它注册了两种消息类型：

- `LtiResourceLinkRequest` - 标准的课程启动到 FastComments。
- `LtiDeepLinkingRequest` - 允许讲师在课程内放置 FastComments 工具。

不会向 LMS 请求额外的访问令牌。

## 未请求的 LTI Advantage 服务

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | 否 | 该集成不需要课程花名册；用户身份随每次启动传递 |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | 否 | 该集成与成绩簿无关 |
| Deep Linking beyond the standard placement return | 不请求额外数据 | 深度链接仅用于讲师放置该工具；不会列举课程内容 |

## 不收集的数据

除 LTI 本身之外，FastComments 不会从 LMS 或用户处请求或接收以下内容：

| Category | Collected? |
|----------|------------|
| Student grades | 否 |
| Assignment submissions | 否 |
| Attendance records | 否 |
| Full course rosters | 否 |
| Government identifiers | 否 |
| Date of birth | 否 |
| Postal address or phone number | 否 |
| Financial information | 否 |
| LMS administrator credentials | 否 |

## 访问边界

- FastComments 仅接收由 LMS 注册密钥签名的授权 LTI 1.3 启动中的数据。该集成不会回调 LMS 以获取额外信息。
- 启动令牌为一次性且短期有效。重放或过期的令牌将被拒绝。
- LMS 管理员控制工具在其平台中的部署位置。例如，D2L Brightspace 支持按部署的组织单元范围和按部署的安全设置，这允许管理员将工具限制到特定课程或组织单元，而不是全局可用。Moodle、Blackboard、Sakai 和 Schoology 在其 LTI 1.3 实现中提供等效的按部署控制。

## 存储与保留

FastComments 在活动评论服务期间并根据客户配置的保留设置保留 LTI 派生的数据。评论数据存储在加密静态生产存储中。在帐户终止或书面删除请求时，FastComments 会根据适用协议删除或匿名化客户数据。

有关完整的存储和数据处理详细信息，请参阅 <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments 隐私政策</a>。

## 审查频率

任何需要额外声明、范围或 LTI Advantage 服务的新 LTI 功能在发布前都会进行审查，以确认所请求的访问对于所交付的功能是必要且相称的。

## 供安全问卷使用的简短声明

> FastComments 对其 LTI 1.3 集成应用最小权限和数据最小化原则。该集成仅使用用于验证用户（`sub`, `name`, `email`, `picture`）、确定其角色以及识别评论所属课程和资源的 LTI 启动声明。FastComments 不请求 Names and Role Provisioning Services、Assignment and Grade Services、成绩簿数据、出勤、完整花名册或 LMS 管理访问权限。LMS 管理员保留对工具可用的组织单元、课程和部署的控制权。