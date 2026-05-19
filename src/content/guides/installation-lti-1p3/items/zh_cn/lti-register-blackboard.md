Blackboard Learn SaaS 和 Ultra 支持 LTI 1.3 动态注册。

#### 打开工具提供者屏幕

1. 以系统管理员身份登录 Blackboard。
2. 导航到 **Administrator Panel** > **Integrations** > **LTI Tool Providers**。
3. 点击 **Register LTI 1.3 / LTI Advantage Tool**。

如果你只看到“Register LTI 1.1 Provider”，说明你的 Blackboard 版本尚不支持 LTI 1.3——请升级或联系 Blackboard 支持。

#### 粘贴 URL

将 FastComments 注册 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>) 粘贴到 **Client ID** / **Registration URL** 字段（Blackboard 的标签因版本而异）。提交。

Blackboard 会与 FastComments 执行注册握手并显示确认屏幕。

#### 批准并启用

Blackboard 默认将新注册的工具标记为 **Approved but excluded**：

1. 在工具提供者列表中找到 FastComments 条目。
2. 打开菜单并选择 **Edit**。
3. 将 **Tool Status** 设置为 **Approved**。
4. 在 **Institution Policies** 下，查看发送了哪些用户数据（姓名、电子邮件、角色）。保存。

该工具现在可供教师在向课程添加内容时使用。