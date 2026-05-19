本页介绍在管理员已注册工具并创建部署后，如何将 FastComments 添加到 Brightspace 课程中。如果尚未注册该工具，请先参阅 D2L 注册指南。

<div class="screenshot white-bg">
    <div class="title">将 FastComments 作为 Brightspace 单元主题嵌入</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments 在 Brightspace 单元内运行，显示线程化评论和 @ 提及选择器" />
</div>

Brightspace 提供两种内容创作体验：**Classic Content** 和 **New Content Experience**（也称为 **Lessons**）。两者都支持 FastComments，但菜单路径不同。下面的每个部分都会在两者分歧处进行说明。

#### 找到 FastComments 工具

FastComments 工具在课程内容编辑器中出现在两个位置：

1. 活动选择器，通过模块/单元的 **Add Existing** 按钮访问（旧版本 Brightspace 标为 **Add Existing Activities**）。在当前 Brightspace 版本中，FastComments 会直接出现在选择器中；旧版本则将其嵌套在 **External Learning Tools** 子菜单下。任一路径都会将 FastComments 添加为独立主题。
2. HTML 编辑器内的 **Insert Stuff** 对话框下的 **LTI Advantage**。这通过 LTI 深度链接流程将 FastComments 内联嵌入到 HTML 主题中。

如果 FastComments 在任一选择器中都未出现，则说明部署未对包含该课程的组织单元启用。请让您的 Brightspace 管理员打开 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**，打开该部署，并在 **Org Units** 下添加该课程的组织单元（或父级组织单元）。

#### 在模块中将 FastComments 添加为主题

Classic Content:

1. 打开课程并在导航栏中点击 **Content**。
2. 选择应包含讨论的模块（或通过 **Add a module** 新建一个）。
3. 点击 **Add Existing**（旧版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在选择器中点击 **FastComments**。Brightspace 会在模块中创建一个主题并将您返回到内容视图。
5. 点击新主题。使用内联标题编辑器将其重命名为描述性名称，例如 `FastComments Discussion`。

New Content Experience (Lessons):

1. 打开课程并点击 **Content**。
2. 打开应包含讨论的单元和 lesson。
3. 点击 **Add** > **Existing Activity** 并选择 **FastComments**（旧版 Brightspace：嵌套在 **External Learning Tools** 下）。
4. 活动会被添加到 lesson 中。
5. 点击活动标题以重命名。

任何用户（讲师或学生）首次打开该主题时，FastComments 会为该资源链接初始化线程。该线程绑定到资源链接 ID，因此重命名或移动主题不会改变加载的线程。

#### 在 HTML 主题中内联嵌入 FastComments

当您希望评论出现在同一主题页面内的阅读、视频或其他内容下方，而不是作为单独主题时，使用此流程。

1. 在模块/lesson 中打开或创建一个 HTML 主题。
2. 点击 **Edit HTML** 以打开 Brightspace HTML 编辑器。
3. 将光标放在应出现评论线程的位置。
4. 点击 **Insert Stuff** 按钮（编辑器工具栏中的拼图图标）。
5. 在 Insert Stuff 对话框中，向下滚动到 **LTI Advantage** 并点击 **FastComments**。
6. FastComments 会打开深度链接选择器。确认放置位置（默认选项适用于内容讨论）；点击 **Insert** 或 **Continue**。
7. Brightspace 会返回 HTML 编辑器并显示表示 LTI 启动的占位块。点击主题上的 **Save and Close**。

当主题加载时，Brightspace 会将该占位块替换为一个通过 LTI 自动启动 FastComments 的 iframe。学生会在页面内看到讨论线程。

一个 HTML 主题可以包含多个深度链接的 FastComments 嵌入。每个嵌入都会产生各自的线程，因为每个深度链接都会生成不同的资源链接 ID。

#### 模块主题 与 内联快速链接（Inline Quicklink）

在以下情况下选择“模块主题”方式：

- 讨论是该模块步骤的主要活动。
- 您希望该主题出现在 Brightspace 的目录、完成跟踪和 Class Progress 中。

在以下情况下选择“内联嵌入”方式：

- 评论应位于同一页面的其他内容下方。
- 您不希望目录中出现单独的、可完成跟踪的条目。

#### 可见性、草稿与发布条件

新的 FastComments 主题默认对学生可见。若想在设置期间隐藏它：

1. 在内容编辑器中，点击主题标题（Classic）或活动上的三点菜单（New Content Experience）。
2. 将状态设为 **Draft**（Classic），或关闭 **Visibility**（New Content Experience）。

草稿主题对学生不可见。讲师和助教仍能看到它们并带有 “Draft” 徽章。

要将主题限制为特定小组或班级部分：

1. 打开主题。
2. 点击主题标题菜单 > **Edit Properties In-place**（Classic），或 **Edit** > **Restrictions**（New Content Experience）。
3. 在 **Release Conditions** 下点击 **Create**。
4. 选择 **Group enrollment** 或 **Section enrollment**，选择相应的小组/部分，然后保存。

发布条件会与 FastComments 自身的角色映射叠加。无法查看该主题的学生不会获得 LTI 启动。

#### 学生首次启动时看到的内容

当学生点击主题（或加载包含嵌入的 HTML 主题）时：

1. Brightspace 在后台执行 LTI 1.3 启动。
2. FastComments 接收学生的姓名、邮箱、头像 URL 和 LMS 角色，并自动为其登录。不会出现 FastComments 登录提示。
3. 该资源链接的评论线程会在 Brightspace iframe 内呈现。

启动时的角色映射：

- Brightspace `Administrator` 变为 FastComments **admin**（线程的完整审核、删除、封禁和配置访问权限）。
- Brightspace `Instructor` 变为 FastComments **moderator**（置顶、隐藏、删除、封禁）。
- 所有其他角色（`Learner`、`TeachingAssistant` 等）则成为普通评论者。

评论将归属于学生的 Brightspace 帐户。如果学生在 Brightspace 中编辑其姓名或头像，下一次 LTI 启动会同步更改。

#### Iframe 高度与重设大小（Resize）

FastComments 在每次线程呈现和内容更改（新增评论、展开回复）时都会发送 `org.imsglobal.lti.frameResize` postMessage。Brightspace 监听此消息并调整 iframe 高度，以防止线程被裁剪或出现内部滚动条。

如果 iframe 保持固定且过短的高度：

- 确认课程是在 HTTPS 下加载。Brightspace 的 postMessage 监听器会拒绝混合内容的框架。
- 确认没有浏览器扩展阻止 postMessage 通道。
- 对于 HTML 主题内的内联嵌入，外围 HTML 不应将 iframe 包裹在固定高度容器中。请从父元素中移除任何内联的 `style="height: ..."`。

#### Brightspace 特定的常见问题

**工具未在 Add Existing 选择器中显示。** 此课程所属的组织单元未对该部署启用。管理员需要将该组织单元（或父级）添加到部署的 **Org Units** 列表。仅注册工具本身不足；部署决定哪些课程可以看到该工具。

**启动时出现 `deployment_id` 不匹配。** FastComments 会将首次看到的 `deployment_id` 进行 TOFU 固定。如果管理员删除了原始部署并创建了新部署，则来自新部署的启动会因部署不匹配错误而被拒绝。解决方法是重新注册 FastComments（生成新的注册 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>）并再次运行 Dynamic Registration）；旧的配置记录将被替换。

**工具已启动但显示 “Invalid LTI launch”。** 课程所在的租户/组织结构不在部署覆盖范围内，或部署在注册后被禁用。请重新检查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切换以及该部署的组织单元列表。

**FastComments 内缺少姓名和角色。** Brightspace 会在 LTI 启动中附带 Names and Role Provisioning Services (NRPS) 声明。如果该课程是从旧的 LTI 1.1 链接升级而来，则启动可能缺少 `name` 和 `email` 声明。通过 **Add Existing** 重新添加 FastComments 主题（不要迁移旧链接），以便启动使用 LTI 1.3。

**嵌入显示登录屏幕而非自动 SSO。** 该 HTML 主题是作为指向 FastComments 的普通 <iframe> 插入的，而不是通过 **Insert Stuff** > **LTI Advantage** 插入。普通 iframe 会跳过 LTI 启动，用户将看到面向公众的 FastComments 页面。请删除该 iframe 并通过 Insert Stuff 流程重新插入。