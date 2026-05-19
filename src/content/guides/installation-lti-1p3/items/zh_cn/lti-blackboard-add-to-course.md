一旦管理员将 FastComments 注册为 LTI 1.3 Advantage 工具并批准了机构策略，讲师即可通过标准的 Blackboard 放置点将其添加到课程中。Ultra Course View 与 Original Course View 的具体步骤有所不同，因此下面两个视图都予以说明。

#### Ultra Course View

截至 2026 年，Ultra Course View 是 Blackboard Learn SaaS 的默认视图。

1. 打开课程并转到 **课程内容** 页面。
2. 将鼠标悬停或点击大纲中希望评论线程出现在的位置，然后点击紫色的 **+**（添加内容）按钮。
3. 选择 **内容市场**。内容市场面板列出机构批准的所有 LTI 工具和 Building Block 放置点。
4. 找到 **FastComments** 磁贴并点击它。Blackboard 会在你打开 **+** 菜单的位置创建一个内容项。
5. 对于默认个人设置中 **隐藏学生** 关闭的讲师，项目默认在大纲中以“对学生可见”条目出现。如果你的默认设置为 **隐藏**，则项目会以隐藏状态创建，准备好后你可以在项行上切换可见性选择器。
6. 若要重命名该项，请在大纲中点击标题并输入新标签。学生在大纲中看到的标题与 FastComments 线程标识符是独立的，因此随时重命名都是安全的。

如果你没有看到 **内容市场** 选项，则说明机构已将该放置隐藏。你也可以通过同一 **+** 菜单下的 **更多工具** 中 **LTI 工具** 组访问相同的选择器。

#### Original Course View

Original Course View 在 Learn SaaS 中仍受支持，并且仍然是基于 Q4 2024 CU 发布线的自托管 Learn 9.1 站点的主要体验。

1. 打开课程并进入一个 **内容区**（例如课程菜单中的默认 **信息** 或 **内容** 区域）。
2. 在页面右上角的切换开关将 **编辑模式** 打开。
3. 在操作栏中点击 **创建内容**。
4. 在 **学习工具** 子菜单下，点击 **FastComments**。学习工具子菜单由管理员注册工具后的 LTI 1.3 工具放置填充。如果你看不到它，请参见下面的注意事项部分。
5. 在 **创建 FastComments** 表单上，设置：
   - **名称**：学生在内容区看到的标签。
   - **描述**：显示在嵌入线程上方的可选文本。
   - **允许用户查看此内容**：是/否 可用性切换。
   - **跟踪查看次数**：如果你想要 Blackboard 的每项查看统计，请启用。FastComments 会独立运行自己的分析。
   - **日期和时间限制**：可选的 **显示之后** / **显示直到** 时间窗口。
6. 提交。该工具将作为可点击项出现在内容区中。

#### 在条目或文档内嵌入

在两种课程视图中，讲师都可以通过内容编辑器的 LTI Advantage 按钮将 FastComments 内联嵌入到条目、文档或任何富文本字段的正文中。

Ultra Course View：

1. 创建或编辑一个 **文档**。
2. 在文档正文中点击 **添加内容**，将光标置于希望线程出现的位置。
3. 在编辑器工具栏中打开 **插入内容** 菜单并点击 **内容市场**（LTI Advantage / 深度链接 的入口）。
4. 选择 **FastComments**。FastComments 返回一个深度链接有效负载，Blackboard 会在光标位置将嵌入块插入到文档正文中。
5. 保存文档。学生在滚动到该处时会看到线程以内联方式渲染。

Original Course View：

1. 编辑任何带有富文本正文的条目。
2. 在内容编辑器工具栏中，点击 **添加内容** 的加号图标并选择 **内容市场**（在较旧的 Q4 2024 CU 中标记为 **从外部工具添加内容**）。
3. 选择 **FastComments**。编辑器会插入一个引用深度链接资源的占位块。
4. 提交该条目。

每个深度链接嵌入都会生成其自己的 FastComments 线程，因此一个包含两个嵌入 FastComments 区块的条目会有两个独立的评论流。

#### 可见性、发布条件与小组限制

FastComments 内容项在访问控制规则上与其他 Blackboard 内容项的行为一致。

- Ultra：在项行上点击可见性选择器（**对学生可见**、**对学生隐藏**、**条件可用性**）。条件可用性支持日期/时间窗口、针对成绩簿项目的表现规则以及针对课程小组的成员规则。
- Original：打开该项的上下文菜单并选择 **自适应发布** 或 **自适应发布：高级**，以按日期、成员资格、成绩或审阅状态对工具进行限制。使用该项上的 **设置小组可用性** 可限制到特定课程小组。

FastComments 尊重 Blackboard 的任何门控决定。如果 Blackboard 对某学生隐藏了该项，则该学生永远不会触发 LTI 启动，也不会出现在版主视图中。

#### 成绩簿行为

FastComments 不会通过 LTI Advantage 的 Assignment and Grade Services 报告成绩。不会为 FastComments 内容项自动创建成绩列。

如果你的 Blackboard 租户配置为对每个新内容项自动创建成绩簿列（无论是否有评分元数据），则仍会出现一个空列。要隐藏它：

- Ultra：打开 **成绩簿**，点击列头，选择 **编辑**，并关闭 **向学生显示** 以及 **计入计算**。或者如果机构允许删除未评分项的列，可使用 **删除**。
- Original：打开 **成绩中心**，点击该列的下拉箭头，选择 **对用户隐藏（开/关）**，并可在 **列组织** 下可选 **对讲师视图隐藏**。

#### 学生看到的内容

当学生打开 FastComments 项目或滚动到嵌入块时：

1. Blackboard 向 FastComments 发起 LTI 1.3 消息。学生通过他们的 Blackboard 身份（姓名、电子邮箱、头像、角色）通过 SSO 登录，无需看到登录表单。
2. 评论线程在 iframe 中渲染。线程、回复、提及和反应等功能将根据在 FastComments 中配置的评论小部件设置可用。
3. 他们的评论会归属于他们的 Blackboard 账户。如果学生之后在 Blackboard 中编辑了姓名或照片，下次启动时会更新 FastComments 配置文件。

从 Blackboard 到 FastComments 的角色映射：

- **System Administrator** 和 **Course Builder** 映射到 FastComments 的 **admin**。
- **Instructor** 和 **Teaching Assistant** 映射到 FastComments 的 **moderator**。
- **Student**、**Guest** 和 **Observer** 映射到 FastComments 的 **commenter**。

版主在每条评论的行内会看到版控操作（置顶、隐藏、封禁、删除）控件。

#### 锁定公开访问（推荐）

默认情况下，FastComments 的评论数据是公开可读的。任何能猜出线程 URL 或 API 端点的人都可以查看其评论，即使不在 Blackboard 中也一样。对于课程讨论，你几乎肯定希望将查看权限限制为仅限已注册的学生。

打开你的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">小部件自定义页面</a>，创建一条启用 **查看评论需 SSO** 的规则，然后将安全级别设置为 **Secure SSO**，这样线程只能通过签名的 LTI 启动加载。

参见 [使用单点登录保护评论线程](/guide-customizations-and-configuration.html#sso-require-to-view-comments) 以获取完整的操作步骤，包括如何将规则限定到单个域或页面。

#### 线程范围

FastComments 通过 **（Blackboard 主机，课程 ID，资源链接 ID）** 对每个线程进行范围划分。同一课程中的两个 FastComments 项会产生两个线程。相同的项目跨两个课程外壳复制（例如通过课程复制）会产生两个线程，因为 Blackboard 在复制过程中会发放新的资源链接 ID。要在课程复制间保持共享线程，请在复制前使用深度链接并在 FastComments 中配置显式线程 URN。

#### Blackboard 特定注意事项

**Build Content 菜单（Original）或内容市场（Ultra）中缺少 FastComments 磁贴。** 管理员批准了该工具，但留下了阻止相关放置的机构策略。前往 **管理员面板** > **集成** > **LTI 工具提供者**，编辑 FastComments 条目，确认已启用 **课程内容工具**（Original）以及 **课程内容工具 - 允许学生** / **深度链接内容工具**（Ultra）这些放置点。保存并刷新课程页面。

**启动时出现“工具未为此上下文配置”或“工具未部署”错误。** 在动态注册期间注册的部署范围与课程所属的机构上下文不匹配。在 Blackboard 的工具提供者条目中，验证 **部署 ID** 是否与 FastComments 在此租户的 LTI 1.3 配置页面上显示的值相匹配。如果不同，请删除该放置并从新的注册 URL 重新运行动态注册（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>）。

**Iframe 高度看起来固定或内容被截断。** 某些 Blackboard 租户附带严格的内容安全策略（CSP），会阻止默认的 LTI iframe-resize postMessage。FastComments 发出 Canvas 风格的 `lti.frameResize` 消息和 IMS 规范形式的 `org.imsglobal.lti.frameResize` 消息以最大化兼容性，但租户级别的 CSP 覆盖会阻止父窗口监听器。请你的管理员确认 `*.fastcomments.com` 已列入 LTI 工具允许列表，并且没有自定义 CSP 头部剥离 postMessage 事件。然后无需进一步配置即可恢复调整大小功能。

**课程复制会复制线程。** Blackboard 在课程复制时为 LTI 放置发放新的资源链接 ID，因此被复制的课程会从空线程开始。这是预期行为。如果你需要被复制的课程继承原始线程，请在复制前使用深度链接并配置显式线程 URN，或联系 FastComments 支持以批量重映射线程 ID。

**学生启动时看到通用 Blackboard 错误。** 原因通常是缺失或过期的 `email` 声明。确认机构策略中为 FastComments 启用了 **角色**、**姓名** 和 **电子邮件地址**（位于 **要发送的用户字段** 下）。保存后，在新的浏览器会话中重新启动。