D2L Brightspace 通过 LTI Advantage 管理界面提供动态注册。您需要管理员权限。

#### 打开注册界面

1. 以管理员身份登录到您的 Brightspace 实例。
2. 导航到 **Admin Tools** > **Manage Extensibility** > **LTI Advantage**。
3. 点击 **Register**。 (直接网址为 `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`。)

#### 粘贴 URL

您会看到一个注册表单。关键字段是 **Tool initiation registration endpoint**（某些 Brightspace 版本将其标记为 "Tool Initiation Registration URL"）。

将 FastComments 注册 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>) 粘贴到该字段。将其他字段留空 - 它们会在注册握手过程中由 FastComments 自动填充。

点击 **Register**。

#### 批准该工具

Brightspace 会打开一个弹出窗口与 FastComments 通信、交换密钥并显示确认屏幕。注册完成后弹出窗口会自动关闭。

新工具会出现在您的 LTI Advantage 工具列表中。默认情况下 Brightspace 会将新工具标记为 **disabled** - 将切换项切换为 **enabled**，以便您的课程可以使用它。

#### 添加部署

在 Brightspace 中，LTI 工具在可用于课程之前需要一个 **deployment**：

1. 打开新注册的 FastComments 工具。
2. 点击 **View Deployments** > **New Deployment**。
3. 为部署命名（例如 "FastComments - All Courses"），选择其应可用的组织单元，然后保存。

通过此部署首次启动后，FastComments 会将 `deployment_id` 固定到其配置记录 - 在同一客户端下来自不同部署的后续启动将被拒绝，除非您重新注册。

---