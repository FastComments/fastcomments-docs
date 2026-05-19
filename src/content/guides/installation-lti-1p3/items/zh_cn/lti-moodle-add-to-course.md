本指南介绍在网站管理员已注册该工具并将其设置为在活动选择器中显示后，如何将 FastComments 添加到 Moodle 4.x 课程。如果尚未注册 FastComments，请先参阅 Moodle 注册指南。

#### 在编辑模式中打开课程

1. 使用课程的 Editing Teacher（或更高权限）身份登录 Moodle。
2. 打开课程。
3. 在课程页眉右上角使用开关将 **Edit mode** 打开。

Moodle 4.x 用全屏活动选择器对替代了 3.x 的旧版“Add an activity or resource”下拉菜单。Moodle 4.5 保持相同的选择器，但在顶部添加了一个已加星标/收藏行，因此将 FastComments 固定一次以后，在以后章节中更容易访问。

#### 添加 FastComments 活动

1. 滚动到讨论所属的课程章节（主题或周）。
2. 点击该章节底部的 **Add an activity or resource**。
3. 在选择器对话框中，选择 **FastComments**。如果看不到它，请跳到下面的注意事项部分。

活动设置表单将打开。以下字段重要：

- **Activity name**（必填）。显示在课程页面和成绩册中。示例： `Week 3 Discussion`。
- **Activity description**。可选的简介文本，呈现在评论线程上方。
- **Show description on course page**。如果希望在不点击活动的情况下显示描述，请勾选此项。
- **Preconfigured tool**。设置为 `FastComments`（从选择器启动时会自动选中）。不要更改。
- **Launch container**。设置为 **New window**。有关“Same window”在某些 Moodle 部署中会导致问题的原因，请参阅注意事项部分。
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**。保持为空。动态注册在站点级别处理这些项。

向下滚动并点击 **Save and return to course**（或 **Save and display** 以立即打开该活动）。

该活动将以 FastComments 图标的行出现在章节中。学生点击该行即可打开评论线程。

#### 在编辑器中以内嵌方式嵌入 FastComments

对于 Page、Book 章节、Lesson 或任何使用 Atto 或 TinyMCE 编辑器的资源中的线程：

1. 以编辑模式打开该资源。
2. 将光标放在希望插入线程的位置。
3. 在编辑器工具栏中，点击 **LTI** / **External tool** 按钮。在 Atto 中标为“Insert LTI Advantage content”。在 TinyMCE（Moodle 4.3+ 的默认编辑器）中，它位于 **More** 菜单下的 **External tools**。
4. 在工具列表中选择 **FastComments**。
5. FastComments 会打开一个深度链接选择器。确认线程标题后点击 **Embed**。
6. 编辑器将插入一个 LTI 占位块。保存资源。

每个嵌入实例都是一个独立的线程，基于深度链接内容项 ID 进行键控，因此一个包含三个 FastComments 嵌入的页面会产生三个独立的线程。

#### 限制访问与小组设置

FastComments 活动适用标准的 Moodle 活动设置：

- **Common module settings** > **Group mode**。将其设置为 **Separate groups** 或 **Visible groups** 本身不会将 FastComments 拆分为每组独立的线程。Moodle 的小组模式仅过滤成绩册和成员列表。要为每个小组运行单独线程，请为每个小组添加一个 FastComments 活动，并使用 **Restrict access** 将每个活动限定范围。
- **Restrict access** > **Add restriction**。支持标准的 Moodle 条件：**Date**、**Grade**、**Group**、**Grouping**、**User profile** 以及嵌套的限制集合。使用 **Group** 将 FastComments 活动锁定到单个小组。
- **Activity completion**。如果希望跟踪完成情况，请设置为 **Students must view this activity to complete it**。除了启动外，FastComments 目前不会向 Moodle 报告其他完成事件。

#### 角色映射

FastComments 读取 Moodle 在每次启动时发送的 LTI `roles` 声明并按如下方式映射：

- Moodle **Manager** 或 **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** 或 **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> 只读

管理员可以删除任何评论、封禁用户并编辑线程设置。版主可以删除并批准他们所启动的线程内的评论。自定义的 Moodle 角色将继承其克隆来源原型的映射。

#### 学生看到的内容

学生点击 FastComments 活动（或滚动到 Page 或 Book 内部的嵌入块）。Moodle 会通过 LTI 向 FastComments 发送他们的身份信息：

- 无需登录屏幕。FastComments 使用 Moodle 帐户为他们登录。
- 他们的显示名称、电子邮件和头像来自 Moodle。
- 线程的范围为 `(Moodle site, course, resource link ID)`，因此相同的活动如果复制到另一个课程会得到一个新的线程。
- 线程化回复、投票和通知与独立的 FastComments 线程工作方式相同。

#### 锁定公开访问（推荐）

默认情况下，FastComments 的评论数据是公开可读的。任何能猜到线程 URL 或 API 端点的人，即便在 Moodle 之外，也可以查看其评论。对于课程讨论，您几乎肯定需要将查看权限限制为仅限已注册的学生。

打开您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">小部件自定义页面</a>，创建一个启用了 **Require SSO To View Comments** 的规则，然后将安全级别设置为 **Secure SSO**，这样线程只能通过已签名的 LTI 启动加载。

参见 [使用单点登录保护评论线程](/guide-customizations-and-configuration.html#sso-require-to-view-comments) 获取完整操作步骤，包括如何将规则限定到单个域名或页面。

#### Moodle 注意事项

**FastComments 在活动选择器中缺失。** 站点管理员已注册该工具，但未将 **Tool configuration usage** 设置为 **Show in activity chooser and as a preconfigured tool**。在 **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > 点击 FastComments 方块上的齿轮图标进行修复。

**设置为“Same window”时启动失败或显示空白框架。** Moodle 的会话 cookie 默认使用 `SameSite=Lax`，某些浏览器会在 LTI 1.3 用于从 FastComments 返回的跨站 POST 上剥离这些 cookie。请在活动中将 **Launch container** 设置为 **New window**。对于嵌入在 Page 或 Book 中的 FastComments，这是一项硬性要求，因为编辑器嵌入的启动路径始终会弹出新窗口。

**`iss` 声明是 Moodle 站点 URL，而不是租户 ID。** FastComments 使用 Moodle 站点 URL（`wwwroot` 配置值）作为 LTI 的发行者。如果您的 Moodle 实例迁移到新域或更改了 `wwwroot`，现有的 FastComments 线程将仍然绑定到旧的发行者，并且不会与新的启动匹配。如有需要，请针对新 URL 重新注册该工具并通过 FastComments 管理端迁移线程。

**活动备份与恢复。** 备份课程并将其恢复到新课程会创建新的资源链接 ID，因此恢复后的 FastComments 活动会以空线程开始。原课程保留原始线程。这是预期行为，不是 bug。

**Moodle 4.5 的 TinyMCE 默认设置。** Moodle 4.5 在新安装中将 TinyMCE 作为默认编辑器。External tool 按钮位置位于 **More**（`...`）菜单下，而不是主工具栏。较早从 4.1 升级的站点会保留 Atto，除非管理员更改了默认设置。