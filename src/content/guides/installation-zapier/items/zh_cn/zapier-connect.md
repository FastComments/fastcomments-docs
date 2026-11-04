## 连接您的账户

1. 在 Zapier 中，将 FastComments 步骤添加到 Zap，或在 Zapier 应用目录中打开 FastComments 应用页面。
2. 选择 **Sign in to FastComments**。Zapier 会先询问您的区域：除非您的账户是在欧盟区域（`eu.fastcomments.com`）创建的，否则请选择 **United States**。
3. 会打开一个 FastComments 窗口。如果您尚未登录，请先登录。
4. 查看授权同意页面。它会显示 Zapier 应用、将要连接的账户，以及所请求的权限（读取和写入）。选择 **Approve**。
5. Zapier 会保存该连接，并使用您的站点名称和用户名对其进行标记。

该连接使用 OAuth。不会将任何 API 密钥复制到 Zapier 中，而且 Zapier 持有的令牌仅对您批准的账户有效。

## 谁可以连接

批准连接的人必须是 FastComments 账户上的 **API 管理员**。账户所有者拥有此权限；其他团队成员可以在“用户”页面上被授予此权限。没有此权限的人会看到“您没有权限”页面，而不是授权同意表单。

## 连接正确的站点

授权同意页面连接的是您当前登录的账户。如果您管理多个账户，请在批准之前通过账户切换器切换到正确的账户，或使用授权同意页面上的 **switch account** 链接。Zapier 中的连接标签会显示站点名称，因此错误的选择很容易被发现。

## 查看和撤销访问权限

每个连接都会显示在 FastComments 控制台的 **Connected Apps** 下，包括其拥有的权限以及上次使用的时间。在该处撤销连接会立即断开 Zapier；任何使用该连接的 Zap 都会停止运行，直到重新连接。您也可以在 Zapier 端的 **My Apps** 下移除该连接。