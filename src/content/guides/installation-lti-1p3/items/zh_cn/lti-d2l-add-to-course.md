本页介绍在管理员已注册工具并创建部署后，如何将 FastComments 添加到 Brightspace 课程中。如果工具尚未注册，请先参阅 D2L 注册指南。

Brightspace 提供两种内容创作体验：**Classic Content**（经典内容）和 **New Content Experience**（也称为 **Lessons，新内容体验**）。两者都可以调用 FastComments，但菜单路径不同。下文每一节在分歧处都会同时覆盖两者。

#### 定位 FastComments 工具

FastComments 工具会出现在课程内容编辑器的两个位置：

1. 活动选择器，可通过模块/单元的 **Add Existing** 按钮（旧版 Brightspace 标为 **Add Existing Activities**）访问。当前 Brightspace 版本中 FastComments 会直接在选择器中显示；旧版本将其嵌在 **External Learning Tools** 子菜单下。任一路径都会将 FastComments 作为独立主题添加。
2. HTML 编辑器内的 **Insert Stuff** 对话框中的 **LTI Advantage** 下。此方式通过 LTI 深度链接流程将 FastComments 内嵌到 HTML 主题中。

如果 FastComments 在任一选择器中都未显示，则说明该部署未对包含课程的组织单元启用。请让你的 Brightspace 管理员打开 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**，打开部署，然后在 **Org Units** 下将该课程的组织单元（或父组织单元）添加进去。

#### 在模块中将 FastComments 添加为主题

经典内容（Classic Content）：

1. 打开课程并在导航栏中点击 **Content**。
2. 选择应包含讨论的模块（或通过 **Add a module** 创建一个模块）。
3. 点击 **Add Existing**（旧版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在选择器中点击 **FastComments**。Brightspace 会在模块中创建一个主题并返回到内容视图。
5. 点击新主题。使用内联标题编辑器将其重命名为描述性名称，例如 `FastComments Discussion`。

新内容体验（New Content Experience）：

1. 打开课程并点击 **Content**。
2. 打开应包含讨论的单元和课件。
3. 点击 **Add** > **Existing Activity** 并选择 **FastComments**（旧版 Brightspace：嵌在 **External Learning Tools** 下）。
4. 该活动会被添加到课件中。
5. 点击活动标题以重命名。

首次由任何用户（教师或学生）打开该主题时，FastComments 会为该资源链接初始化线程。线程与资源链接 ID 绑定，因此重命名或移动主题不会改变加载的线程。

#### 在 HTML 主题中内嵌 FastComments

当你希望评论出现在同一主题页面中阅读材料、视频或其他内容下方，而不是作为单独主题时，请使用此流程。

1. 在模块/课件中打开或创建一个 HTML 主题。
2. 点击 **Edit HTML** 以打开 Brightspace HTML 编辑器。
3. 将光标放在评论线程应出现的位置。
4. 点击 **Insert Stuff** 按钮（编辑器工具栏中的拼图图标）。
5. 在 Insert Stuff 对话框中，滚动到 **LTI Advantage** 并点击 **FastComments**。
6. FastComments 会打开深度链接选择器。确认放置位置（默认选项适用于内容讨论）；点击 **Insert** 或 **Continue**。
7. Brightspace 会返回到 HTML 编辑器，并显示一个表示 LTI 启动的占位块。点击主题上的 **Save and Close**。

当主题加载时，Brightspace 会用自动通过 LTI 启动 FastComments 的 iframe 替换占位符。学生会在页面内看到讨论线程。

单个 HTML 主题可以包含多个深度链接的 FastComments 嵌入。每个嵌入都会得到自己的线程，因为每个深度链接都会生成一个不同的资源链接 ID。

#### 模块主题 与 内联快速链接 的选择

请选择“模块主题”方式，当：

- 该讨论是模块中该步骤的主要活动。
- 你希望该主题出现在 Brightspace 的目录、完成跟踪和 Class Progress 中。

请选择“内联嵌入”方式，当：

- 评论应位于同一页面上的其他内容下方。
- 你不希望目录中出现单独的、可完成追踪的条目。

#### 可见性、草稿和发布条件

新的 FastComments 主题默认对学生可见。要在设置期间将其隐藏：

1. 在内容编辑器中，点击主题标题（经典内容）或活动上的三点菜单（新内容体验）。
2. 将状态设置为 **Draft**（经典内容）或关闭 **Visibility**（新内容体验）。

草稿主题对学生不可见。教师和助教仍会看到带有“Draft”徽章的主题。

要将主题限制为特定组或分区：

1. 打开主题。
2. 点击主题标题菜单 > **Edit Properties In-place**（经典内容）或 **Edit** > **Restrictions**（新内容体验）。
3. 在 **Release Conditions** 下，点击 **Create**。
4. 选择 **Group enrollment** 或 **Section enrollment**，选择组/分区并保存。

发布条件将与 FastComments 自身的角色映射叠加。无法查看该主题的学生将无法获得 LTI 启动。

#### 学生首次启动时看到的内容

当学生点击主题（或加载带有嵌入的 HTML 主题）时：

1. Brightspace 在后台执行 LTI 1.3 启动。
2. FastComments 接收学生的 `name`、`email`、头像 URL 和 LMS 角色，并自动为其登录。不会出现 FastComments 登录提示。
3. 该资源链接的评论线程在 Brightspace iframe 内呈现。

启动时的角色映射：

- Brightspace 的 `Administrator` 会成为该线程的 FastComments **管理员**（拥有完整的审核、删除、封禁和配置访问权限）。
- Brightspace 的 `Instructor` 会成为 FastComments **版主**（置顶、隐藏、删除、封禁）。
- 所有其他角色（`Learner`、`TeachingAssistant` 等）成为普通评论者。

评论会归因于学生的 Brightspace 账号。如果学生在 Brightspace 中编辑了姓名或头像，下一次 LTI 启动会同步该更改。

#### iframe 高度与重设大小

FastComments 在每次线程呈现和内容更改（新评论、展开回复）时都会发送 `org.imsglobal.lti.frameResize` postMessage。Brightspace 监听此消息并调整 iframe 高度，以防线程被截断或出现内部滚动条。

如果 iframe 保持固定较短高度：

- 确认课程是否通过 HTTPS 加载。Brightspace 的 postMessage 监听会拒绝混合内容的框架。
- 确认没有浏览器扩展阻止 postMessage 通道。
- 对于 HTML 主题中的内联嵌入，外围 HTML 不应将 iframe 包裹在固定高度的容器中。请从父元素中移除任何内联 `style="height: ..."`。

#### Brightspace 特定问题提示

**工具未在 Add Existing 选择器中显示。** 说明该部署未针对该课程的组织单元启用。管理员需要将该组织单元（或父组织单元）添加到部署的 **Org Units** 列表中。仅注册工具本身不足；部署决定了哪些课程可以看到该工具。

**启动时出现 `deployment_id` 不匹配。** FastComments 会把它第一次看到的 `deployment_id` 当作可信来源固定（TOFU）。如果管理员删除了原始部署并创建了一个新部署，则来自新部署的启动会因部署不匹配错误被拒绝。修复方法是重新注册 FastComments（生成新的注册 URL 并再次运行动态注册）；旧的配置记录将被替换。

**工具启动但显示“Invalid LTI launch”。** 课程所在的租户/组织结构不在部署覆盖范围内，或部署在注册后被禁用。请重新检查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切换以及该部署的组织单元列表。

**FastComments 中缺少姓名和角色。** Brightspace 会在 LTI 启动中携带 Names and Role Provisioning Services (NRPS) 声明。如果课程是从旧的 LTI 1.1 链接升级而来，启动可能缺少 `name` 和 `email` 声明。请通过 **Add Existing** 重新添加 FastComments 主题（不要迁移旧链接），以便启动使用 LTI 1.3。

**嵌入显示登录界面而非自动 SSO。** 该 HTML 主题是作为指向 FastComments 的普通 `<iframe>` 插入的，而不是通过 **Insert Stuff** > **LTI Advantage**。普通 iframe 会跳过 LTI 启动，用户会进入面向公众的 FastComments 页面。删除该 iframe 并通过 Insert Stuff 流程重新插入。