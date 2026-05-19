**使用 Moodle？** 我们还为 FastComments 发布了专门的 Moodle 插件，提供比 LTI 1.3 更紧密的集成（成绩同步钩子、更深入的活动报告、本地 Moodle 设置界面）。请参阅 <a href="/guide-installation-moodle.html" target="_blank">Moodle 插件安装指南</a>。如果你希望进行一次注册覆盖其他 LMS，或你的 Moodle 管理员不愿安装第三方插件，下面的 LTI 1.3 流程是正确的选择。

Moodle 4.0+ 支持通过 External Tool 插件进行 LTI 1.3 的动态注册。

#### 打开工具管理界面

1. 以站点管理员身份登录 Moodle。
2. 导航到 **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**。

#### 粘贴 URL

你会看到一个标为 **Tool URL** 的卡片。将 FastComments 的注册 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>) 粘贴到文本字段，然后点击 **Add LTI Advantage**。

Moodle 会打开一个注册界面，显示该工具的身份信息及其请求的权限。检查无误后点击 **Activate**（或 **Register**，取决于 Moodle 版本）。

注册完成后弹窗会关闭；新的 FastComments 工具会以 **Active** 状态出现在 **Tools** 列表中。

#### 使其可用

默认情况下，Moodle 会将新工具添加到“Course tools”列表，但不会在活动选择器中显示它们。要在课程范围内公开 FastComments：

1. 点击 FastComments 磁贴上的齿轮图标。
2. 在 **Tool configuration usage** 下，选择 **Show in activity chooser and as a preconfigured tool**。
3. 保存。

教师现在可以通过 **Add an activity or resource** > **FastComments** 将 FastComments 添加到任何课程。