一旦 FastComments 在平台上注册完成，教师即可使用平台的标准外部工具流程将其添加到课程内容中。本页涵盖 Sakai 23.x 和 Schoology Enterprise。

#### Sakai

**1. 将 FastComments 添加到站点**

站点维护者按站点启用该工具：

1. 打开站点并在左侧导航中点击 **Site Info**。
2. 点击 **Manage Tools**。
3. 滚动到 **External Tools** 列表并将 **FastComments** 切换为开启。
4. 点击 **Continue**，查看工具列表，然后点击 **Finish**。

FastComments 现在会显示为站点左侧导航项。

**2. 重新排序左侧导航项**

转到 **Site Info** > **Tool Order**。将 **FastComments** 拖到所需位置并点击 **Save**。你也可以在此屏幕重命名导航标签并对学生隐藏它。

**3. 在 Lessons 页面内嵌入**

要将 FastComments 直接放在 Lessons 页面内，而不是作为独立的左侧导航工具：

1. 在站点中打开 **Lessons** 工具。
2. 点击 **Add Content** > **Add External Tool**。
3. 从列表中选择 **FastComments**。
4. 如果在注册期间 FastComments 声明了深度链接（Deep Linking），Sakai 会打开工具的内容选择器以便你选择或标记线程。如果未声明深度链接，Sakai 则插入一个默认的启动链接。
5. 保存 Lessons 条目。

每个嵌入实例都会获得自己的线程，作用域限定为该资源链接。

**4. 学生访问的权限调整**

Sakai 通过 Realms 对外部工具的启动进行控制。要确认学生可以启动 FastComments：

1. 以 Sakai 管理员身份登录并打开 **Administration Workspace** > **Realms**。
2. 打开相关的 realm（例如 `!site.template.course` 或具体的站点 realm）。
3. 确认 `access` 角色已启用 `lti.launch`，并且 **external.tools** 组中的角色权限已授予。
4. 保存 realm。

对于站点级别的覆盖，维护者可以在 **Site Info** > **Tool Order** 中按角色隐藏或显示 FastComments，从而调整每个角色的工具可见性。

**5. 学生看到的内容**

学生点击左侧导航的 FastComments 项（或滚动到嵌入的 Lessons 模块）即可直接进入主题评论视图。单点登录 (SSO) 是自动的：Sakai 会在 LTI 启动中发送用户身份信息，FastComments 会以其 Sakai 帐户为其登录。

角色映射：

- Sakai `Instructor` -> FastComments 版主
- Sakai `Admin` (admin in Administration Workspace) -> FastComments 管理员
- Sakai `Student` / `access` -> FastComments 评论者

**6. Sakai 注意事项**

- **Manage Tools 中看不到该工具。** 如果 FastComments 未出现在 External Tools 列表中，Sakai 管理员需要打开工具注册表（**Administration Workspace** > **External Tools** > **FastComments**）并将 **Stealthed** 设置为 `false`。被隐藏（stealthed）的工具不会出现在每站点的 Manage Tools 选择器中。
- **在共享会话的浏览器中启动失败。** Sakai 的门户 CSRF 令牌绑定到浏览器会话。如果学生在不同标签页登录了两个 Sakai 站点或存在过期会话，启动会返回 403。修复方法：关闭其他 Sakai 标签页，登出并重新登录，然后重新启动。若在集群范围内发生此问题，管理员也可以提高 `sakai.csrf.token.cache.ttl`。
- **页面内嵌入的框架问题。** 请确认在 `sakai.properties` 中 `lti.frameheight` 足够大（600 或更高），以免在 Lessons 页面内评论线程被裁剪。

#### Schoology

Schoology Enterprise 有两种安装场景。请在将工具添加到课程前确认适用的场景。

**1. 两种安装场景**

- **(a) 企业级安装。** Schoology 系统管理员已在组织层级安装 FastComments 并将其分配给所有课程或特定的课程模板。教师可跳过安装，直接进入 “Add Materials”。
- **(b) 教师自助安装。** 教师可从 **Course Options** > **External Tools** > **Install LTI Apps** 在单个课程中安装该工具。自助安装要求系统管理员先在组织层级批准 FastComments 应用。

**2. 将 FastComments 添加为课程材料**

在课程内：

1. 打开课程并转到 **Materials**。
2. 点击 **Add Materials** > **Add File/Link/External Tool**。
3. 选择 **External Tool**。
4. 从已注册的工具列表中选择 **FastComments**。
5. 设置一个 **Name**（学生在材料列表中看到的名称）和可选的 **Description**。
6. 保持 **Enable Grading**（成绩回传）为 **OFF**。FastComments 不会向 Schoology 回传成绩，因此启用成绩回传会创建一个空的成绩簿列。
7. 点击 **Submit**。

该材料现在出现在课程材料列表中，点击后会打开 FastComments 线程。

**3. 通过富文本编辑器内联嵌入**

如果系统管理员在注册期间为 FastComments 启用了深度链接（Deep Linking）放置，教师可以在任何富文本字段（作业说明、页面正文、讨论提示）中嵌入评论线程：

1. 在目标页面打开富文本编辑器。
2. 点击工具栏中的 **External Tool**（拼图图标）按钮。
3. 选择 **FastComments**。
4. 在深度链接对话框中配置嵌入并点击 **Insert**。
5. 保存页面。

如果富文本编辑器中没有 External Tool 按钮，说明该租户针对该工具禁用了深度链接。见下文的注意事项。

**4. 可见性与分班分配**

Schoology 通过 Course Options 按分班控制工具的可用性：

1. 在课程中点击 **Course Options** > **External Tools**。
2. 对于每个已安装的 LTI 应用，你可以控制它是对课程中的所有分班可见，还是仅对特定分班可见。
3. 若要将 FastComments 限制在某些分班，取消勾选不应看到该工具的分班。
4. 分班级别的访问也决定哪些分班在 **Add Materials** > **External Tool** 中可以看到 FastComments 条目。

**5. 学生看到的内容**

学生点击 FastComments 材料（或滚动到内联嵌入处）即可进入主题讨论。单点登录通过 Schoology 的 LTI 启动自动完成，使用他们的 Schoology 帐户登录。

角色映射：

- Schoology `Administrator` -> FastComments 管理员
- Schoology `Instructor` -> FastComments 版主
- Schoology `Student` -> FastComments 评论者

**6. Schoology 注意事项**

- **仅限企业版。** 个人和免费 Schoology 帐户无法安装 LTI 1.3 工具。如果你的租户使用免费等级，Course Options 中不会出现 **External Tools** 选项。请升级到 Schoology Enterprise 以使用 FastComments。
- **租户默认禁用深度链接。** 某些 Schoology 租户在组织层面限制深度链接放置。若出现这种情况，教师只能看到 **Add Materials** > **External Tool** 流程，而富文本编辑器中不会出现 External Tool 按钮。要启用内联嵌入，系统管理员需前往 **System Settings** > **Integration** > **LTI 1.3** > **FastComments** 并启用 **Content Item / Deep Linking** 放置，然后保存。
- **分班分配的覆盖。** 如果 FastComments 在企业层级被分配但教师在 **Add Materials** 中看不到它，说明该课程的分班在组织层级分配中被排除了。请让系统管理员将该分班添加到 FastComments 应用的分配中。
- **材料名称与线程身份不同。** 在 Schoology 中重命名材料并不会移动评论线程。线程以 LTI 资源链接 ID 为键，因此重命名不会改变线程；删除并重新创建材料会创建一个新的空线程。