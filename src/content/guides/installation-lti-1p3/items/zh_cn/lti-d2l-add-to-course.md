本页介绍在管理员已注册该工具并创建部署后，如何将 FastComments 添加到 Brightspace 课程中。如果该工具尚未注册，请先参阅 D2L 注册指南。

<div class="screenshot white-bg">
    <div class="title">FastComments 嵌入为 Brightspace 单元主题</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace 提供两种内容创作体验：**Classic Content** 和 **New Content Experience**（也称为 **Lessons**）。两者都支持 FastComments，但菜单路径有所不同。下面每一节在分歧处都会分别说明。

#### 定位 FastComments 工具

FastComments 工具在课程内容编辑器中有两个出现位置：

1. 活动选择器（activity picker），通过模块/单元的 **Add Existing** 按钮访问（较旧的 Brightspace 版本标为 **Add Existing Activities**）。在当前 Brightspace 版本中，FastComments 会直接显示在选择器中；较旧版本将其嵌在 **External Learning Tools** 子菜单下。任一路径都会将 FastComments 添加为独立主题。
2. HTML 编辑器内的 **Insert Stuff** 对话框下的 **LTI Advantage**。这会通过 LTI 深度链接流程将 FastComments 以内联方式嵌入到 HTML 主题中。

如果 FastComments 未出现在任一选择器中，说明部署未对包含该课程的组织单元启用。请让您的 Brightspace 管理员打开 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**，打开相应部署，并在 **Org Units** 下为该部署添加课程所属的组织单元（或其父级组织单元）。

#### 在模块中将 FastComments 添加为主题

Classic Content：

1. 打开课程并在导航栏中点击 **Content**。
2. 选择应包含讨论的模块（或通过 **Add a module** 创建一个）。
3. 点击 **Add Existing**（旧版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在选择器中点击 **FastComments**。Brightspace 会在模块中创建一个主题并返回内容视图。
5. 点击新主题。使用内联标题编辑器将其重命名为诸如 `FastComments Discussion` 的描述性名称。

New Content Experience (Lessons)：

1. 打开课程并点击 **Content**。
2. 打开应包含讨论的单元和课时（unit 和 lesson）。
3. 点击 **Add** > **Existing Activity** 并选择 **FastComments**（旧版 Brightspace：嵌在 **External Learning Tools** 下）。
4. 活动会被添加到课时中。
5. 点击活动标题以重命名。

任何用户（教师或学生）首次打开该主题时，FastComments 会为该资源链接初始化线程。该线程绑定到资源链接 ID，因此重命名或移动主题不会改变加载的线程。

#### 在 HTML 主题内以内联方式嵌入 FastComments

当您希望评论出现在同一主题页面的阅读、视频或其他内容下方（而不是作为单独主题）时，使用此流程。

1. 在模块/课时中打开或创建一个 HTML 主题。
2. 点击 **Edit HTML** 打开 Brightspace HTML 编辑器。
3. 将光标放在应显示评论线程的位置。
4. 点击 **Insert Stuff** 按钮（编辑器工具栏中的拼图图标）。
5. 在 Insert Stuff 对话框中，滚动到 **LTI Advantage** 并点击 **FastComments**。
6. FastComments 会打开深度链接选择器。确认放置位置（默认选项适用于内容讨论）；点击 **Insert** 或 **Continue**。
7. Brightspace 会返回到 HTML 编辑器，并显示一个表示 LTI 启动的占位块。在主题上点击 **Save and Close**。

当主题加载时，Brightspace 会将占位符替换为通过 LTI 自动启动 FastComments 的 iframe。学生会看到内联的讨论线程。

单个 HTML 主题可以包含多个深度链接的 FastComments 嵌入。每个嵌入都会得到自己的线程，因为每个深度链接都会产生不同的资源链接 ID。

#### 模块主题 与 内联快速链接 的选择

当满足以下情况时，请选择 **模块主题** 方法：

- 该讨论是该模块步骤的主要活动。
- 您希望该主题出现在 Brightspace 的目录、完成跟踪和 Class Progress 中。

当满足以下情况时，请选择 **内联嵌入** 方法：

- 评论应位于同一页面上的其他内容下方。
- 您不希望在目录中创建单独的、可完成跟踪的条目。

#### 可见性、草稿和发布条件

新的 FastComments 主题默认对学生可见。若要在设置期间隐藏它：

1. 在内容编辑器中，点击主题标题（Classic）或活动上的三点菜单（New Content Experience）。
2. 将状态设置为 **Draft**（Classic）或关闭 **Visibility**（New Content Experience）。

草稿主题对学生不可见。教师和助教仍能看到并带有 “Draft” 徽章。

要将主题限制为特定小组或班级部分：

1. 打开主题。
2. 点击主题标题菜单 > **Edit Properties In-place**（Classic）或 **Edit** > **Restrictions**（New Content Experience）。
3. 在 **Release Conditions** 下，点击 **Create**。
4. 选择 **Group enrollment** 或 **Section enrollment**，选择相应的小组/部分并保存。

发布条件会与 FastComments 自身的角色映射叠加。无法查看该主题的学生不会收到 LTI 启动。

#### 学生首次启动时看到的内容

当学生点击主题（或加载包含嵌入的 HTML 主题）时：

1. Brightspace 在后台执行 LTI 1.3 启动。
2. FastComments 收到学生的姓名、电子邮件、头像 URL 和 LMS 角色，并自动为其登录。不会出现 FastComments 登录提示。
3. 该资源链接的评论线程在 Brightspace iframe 内呈现。

启动时的角色映射：

- Brightspace 的 `Administrator` 在该线程中成为 FastComments 的管理员（admin），具有完整的审核、删除、禁止和配置访问权限。
- Brightspace 的 `Instructor` 成为 FastComments 的版主（moderator）（固定、隐藏、删除、禁止）。
- 其他所有角色（`Learner`、`TeachingAssistant` 等）成为普通评论者。

评论会标注为学生的 Brightspace 账户。如果学生在 Brightspace 中编辑其姓名或头像，下一次 LTI 启动会同步该更改。

#### 锁定公开访问（建议）

默认情况下，FastComments 的评论数据是公开可读的。任何能猜到线程 URL 或 API 端点的人都可以查看其评论，即使在 Brightspace 之外。对于课程讨论，您几乎肯定希望将查看权限限制为仅限已注册的学员。

打开您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> 并创建一个启用 **Require SSO To View Comments** 的规则，然后将安全级别设置为 **Secure SSO**，以便线程只能通过签名的 LTI 启动加载。

完整操作请参阅 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)，其中包括如何将规则限定到单个域或页面的完整步骤。

#### Iframe 高度与重设大小

FastComments 会在每次线程呈现和内容更改（新评论、展开回复）时发出 `org.imsglobal.lti.frameResize` postMessage。Brightspace 监听该消息并调整 iframe 高度，以避免线程被裁剪或出现内部滚动条。

如果 iframe 仍保持固定的短高度：

- 确认课程是通过 HTTPS 加载的。Brightspace 的 postMessage 监听会拒绝混合内容的框架。
- 确认没有浏览器扩展阻止 postMessage 通道。
- 对于 HTML 主题内的内联嵌入，外围 HTML 不应将 iframe 包裹在固定高度的容器中。请从父元素中移除任何内联 `style="height: ..."`。

#### Brightspace 特定问题提示

**工具未在 Add Existing 选择器中显示。** 说明该部署未对该课程所属的组织单元启用。管理员需要将该组织单元（或其父级）添加到部署的 **Org Units** 列表中。仅注册工具并不足够；部署决定了哪些课程可以看到该工具。

**启动时出现 `deployment_id` 不匹配。** FastComments 会把首次看到的 `deployment_id` 作为信任首次使用（TOFU）地固定。如果管理员删除了原始部署并创建了新部署，则来自新部署的启动会因部署不匹配而被拒绝。解决方法是重新注册 FastComments（生成新的注册 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>）并再次运行动态注册）；旧的配置记录将被替换。

**工具启动但显示“Invalid LTI launch”。** 课程所在的租户/组织结构不在部署覆盖范围内，或部署在注册后被禁用。重新检查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切换以及部署的组织单元列表。

**FastComments 内缺少姓名和角色。** Brightspace 在 LTI 启动中携带 Names and Role Provisioning Services (NRPS) 声明。如果某课程是从较旧的 LTI 1.1 链接升级而来，启动可能缺少 `name` 和 `email` 声明。通过 **Add Existing** 重新添加 FastComments 主题（不要迁移旧链接），以便启动使用 LTI 1.3。

**嵌入显示登录屏幕而非自动 SSO。** 该 HTML 主题被插入为指向 FastComments 的普通 `<iframe>`，而不是通过 **Insert Stuff** > **LTI Advantage** 插入。普通 iframe 会跳过 LTI 启动，直接将用户带到面向公众的 FastComments 页面。删除该 iframe 并通过 Insert Stuff 流程重新插入。