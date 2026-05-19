一旦 FastComments 在平台上注册，教师即可使用平台的标准外部工具流程将其添加到课程内容中。本页涵盖 Sakai 23.x 和 Schoology Enterprise。

#### 限制公开访问（推荐）

默认情况下，FastComments 的评论数据在任一平台上都是公开可读的。任何能够猜到线程 URL 或 API 端点的人都可以查看其评论，甚至在 Sakai 或 Schoology 之外。对于课程讨论，您几乎肯定希望将查看权限限制为仅限已注册的学生。

打开您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">小部件自定义页面</a>，创建一条规则并启用 **Require SSO To View Comments**，然后将安全级别设置为 **Secure SSO**，以便线程只能通过签名的 LTI 启动加载。

参见 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) 获取完整的演练，包括如何将规则限制到单个域或页面。

#### Sakai

**1. 将 FastComments 添加到站点**

站点维护者按站点启用该工具：

1. 打开站点并在左侧导航中点击 **Site Info**。
2. 点击 **Manage Tools**。
3. 滚动到 **External Tools** 列表并将 **FastComments** 切换为开启。
4. 点击 **Continue**，查看工具列表，然后点击 **Finish**。

FastComments 现在会作为站点左侧导航项出现。

**2. 重新排序左侧导航条目**

转到 **Site Info** > **Tool Order**。将 **FastComments** 拖到所需位置并点击 **Save**。您也可以在此屏幕上重命名导航标签并将其对学生隐藏。

**3. 在 Lessons 页面中内嵌**

若要将 FastComments 直接放置在 Lessons 页面内，而不是作为独立的左侧导航工具：

1. 在站点中打开 **Lessons** 工具。
2. 点击 **Add Content** > **Add External Tool**。
3. 从列表中选择 **FastComments**。
4. 如果 FastComments 在注册期间声明了 Deep Linking，Sakai 会打开该工具的内容选择器，您可以选择或标记线程。如果未声明 Deep Linking，Sakai 会插入默认的启动链接。
5. 保存 Lessons 条目。

每个嵌入实例都有自己的线程，范围限定为该资源链接。

**4. 针对学生访问的权限调整**

Sakai 通过 Realms 控制外部工具的启动。确认学生可以启动 FastComments 的步骤：

1. 以 Sakai 管理员身份登录并打开 **Administration Workspace** > **Realms**。
2. 打开相关的 realm（例如 `!site.template.course` 或特定站点的 realm）。
3. 确认 `access` 角色已启用 `lti.launch`，并且 **external.tools** 组中的角色权限已被授予。
4. 保存 realm。

对于站点级的覆盖，维护者可以在 **Site Info** > **Tool Order** 中通过对各角色隐藏或显示 FastComments 来调整每个角色的工具可见性。

**5. 学生看到的内容**

学生点击 FastComments 左侧导航项（或滚动到嵌入的 Lessons 块）即可直接进入线程评论视图。SSO 为自动：Sakai 在 LTI 启动中发送用户身份，FastComments 使用其 Sakai 帐户为其登录。

角色映射：

- Sakai `Instructor` -> FastComments 版主
- Sakai `Admin` (Administration Workspace 中的管理员) -> FastComments 管理员
- Sakai `Student` / `access` -> FastComments 评论者

**6. Sakai 常见问题**

- **Manage Tools 中看不到该工具。** 如果 FastComments 未出现在 External Tools 列表中，Sakai 管理员需要打开工具注册表（**Administration Workspace** > **External Tools** > **FastComments**）并将 **Stealthed** 设置为 `false`。被 Stealthed 的工具会在每站点的 Manage Tools 选择器中隐藏。
- **在共享会话的浏览器中启动失败。** Sakai 的门户 CSRF 令牌绑定到浏览器会话。如果学生在不同标签页登录了两个 Sakai 站点或存在过期会话，启动会返回 403。解决方法：关闭其他 Sakai 标签页，登出，重新登录，然后重新启动。管理员也可以在群集范围内发生此问题时提高 `sakai.csrf.token.cache.ttl`。
- **嵌入框架问题。** 确认在 `sakai.properties` 中 `lti.frameheight` 足够大（600 或更高），以免在 Lessons 页面中评论线程被截断。

#### Schoology

Schoology Enterprise 有两种安装场景。添加工具到课程之前请确认适用的场景。

**1. 两种安装场景**

- **(a) 企业级安装。** Schoology 系统管理员在组织层面安装了 FastComments 并将其分配给所有课程或特定的课程模板。教师可跳过安装，直接进入 “Add Materials”。
- **(b) 教师自助安装。** 教师从 **Course Options** > **External Tools** > **Install LTI Apps** 将该工具安装到单个课程中。自助安装之前需要系统管理员先在组织层级批准 FastComments 应用。

**2. 将 FastComments 作为课程资料添加**

在课程内：

1. 打开课程并转到 **Materials**。
2. 点击 **Add Materials** > **Add File/Link/External Tool**。
3. 选择 **External Tool**。
4. 从已注册的工具列表中选择 **FastComments**。
5. 设置一个 **Name**（这是学生在资料列表中看到的名称）和可选的 **Description**。
6. 保持 **Enable Grading**（成绩回传）为 **OFF**。FastComments 不向 Schoology 报告成绩，因此启用成绩回传会创建一个空的成绩册列。
7. 点击 **Submit**。

该资料现在出现在课程资料列表中，点击时会打开 FastComments 线程。

**3. 通过富文本编辑器内联嵌入**

如果系统管理员在注册期间为 FastComments 启用了 Deep Linking 放置，教师可以在任何富文本字段（作业说明、页面正文、讨论提示）中嵌入评论线程：

1. 在目标页面打开富文本编辑器。
2. 点击工具栏中的 **External Tool**（拼图图标）图标。
3. 选择 **FastComments**。
4. 在深度链接对话框中配置嵌入并点击 **Insert**。
5. 保存页面。

如果富文本编辑器中未出现 External Tool 按钮，则说明在此租户中禁用了 Deep Linking。见下文的注意事项。

**4. 可见性与小节分配**

Schoology 通过课程选项按小节范围控制工具可用性：

1. 在课程中，点击 **Course Options** > **External Tools**。
2. 对于每个已安装的 LTI 应用，您可以控制它是对课程中所有小节可见还是仅对特定小节可见。
3. 若要将 FastComments 限制到某些小节，请取消选中不应看到该工具的小节。
4. 小节级访问也控制哪些小节可以看到 **Add Materials** > **External Tool** 中的 FastComments 条目。

**5. 学生看到的内容**

学生点击 FastComments 资料（或滚动到内联嵌入处）即可进入线程讨论。SSO 通过他们的 Schoology 帐户随 Schoology 的 LTI 启动自动完成。

角色映射：

- Schoology `Administrator` -> FastComments 管理员
- Schoology `Instructor` -> FastComments 版主
- Schoology `Student` -> FastComments 评论者

**6. Schoology 常见问题**

- **仅限企业版。** 个人和免费 Schoology 帐户无法安装 LTI 1.3 工具。如果您的租户使用的是免费套餐，**Course Options** 中将没有 **External Tools** 选项。升级到 Schoology Enterprise 以使用 FastComments。
- **租户默认禁用 Deep Linking。** 一些 Schoology 租户在组织层面限制 Deep Linking 放置。若出现这种情况，教师在富文本编辑器中只会看到 **Add Materials** > **External Tool** 流程，而不会看到富文本编辑器中的 External Tool 按钮。要启用内联嵌入，系统管理员需转到 **System Settings** > **Integration** > **LTI 1.3** > **FastComments** 并启用 **Content Item / Deep Linking** 放置，然后保存。
- **按小节分配被覆盖。** 如果 FastComments 在企业级分配但教师在 **Add Materials** 中看不到它，则可能是课程的小节在组织级分配中被排除了。请系统管理员将该小节添加到 FastComments 应用分配中。
- **资料名称与线程标识。** 在 Schoology 中重命名资料不会移动评论线程。线程是基于 LTI 资源链接 ID 键控的，因此重命名会保持相同线程；删除并重新创建资料会创建一个新的、空的线程。