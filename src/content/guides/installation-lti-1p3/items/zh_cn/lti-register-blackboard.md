Blackboard Learn SaaS 和 Ultra 支持 LTI 1.3 Dynamic Registration。

#### 打开工具提供者界面

1. 使用系统管理员账号登录 Blackboard。
2. 导航到 **Administrator Panel** > **Integrations** > **LTI Tool Providers**。
3. 点击 **Register LTI 1.3 / LTI Advantage Tool**。

如果您只看到 “Register LTI 1.1 Provider”，说明您的 Blackboard 版本尚不支持 LTI 1.3 —— 请升级或联系 Blackboard 支持。

#### 粘贴 URL

将 FastComments 注册 URL 粘贴到 **Client ID** / **Registration URL** 字段（Blackboard 在不同版本中的标注可能不同）。提交。

Blackboard 会与 FastComments 完成注册握手并显示确认界面。

#### 批准并启用

Blackboard 默认将新注册的工具标记为 **Approved but excluded**：

1. 在工具提供者列表中找到 FastComments 条目。
2. 打开菜单并选择 **Edit**。
3. 将 **Tool Status** 设置为 **Approved**。
4. 在 **Institution Policies** 下，检查将发送的用户数据（姓名、电子邮件、角色）。保存。

当教师向课程添加内容时，该工具现已可用。