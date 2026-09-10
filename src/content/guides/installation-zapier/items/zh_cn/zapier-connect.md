## 连接您的账户

1. 在 Zapier 中，向 Zap 添加一个 FastComments 步骤，或在 Zapier 应用目录中打开 FastComments 应用页面。  
2. 选择 **Sign in to FastComments**。Zapier 会先询问您的地区：除非您的账户是在欧盟地区创建的（`eu.fastcomments.com`），否则请选择 **United States**。  
3. 将打开 FastComments 窗口。如果您尚未登录，请登录。  
4. 查看同意页面。页面显示 Zapier 应用、将要连接的账户以及请求的权限（读取和写入）。选择 **Approve**。  
5. Zapier 会存储该连接，并使用您的站点名称和用户名对其进行标记。  

该连接使用 OAuth。没有 API 密钥被复制到 Zapier，Zapier 持有的令牌仅对您批准的账户有效。

## 谁可以连接

批准连接的人员必须是 FastComments 账户的 **API 管理员**。账户所有者拥有此权限；其他团队成员可以在“用户”页面上被授予此权限。没有此权限的用户会看到“you do not have permission”页面，而不是同意表单。

## 连接正确的站点

同意页面会连接您当前登录的账户。如果您管理多个账户，请在批准之前从账户切换器中切换到正确的账户，或使用同意页面上的 **switch account** 链接。Zapier 中的连接标签会显示站点名称，因此错误的选择很容易被发现。

## 查看和撤销访问

每个连接都会出现在 FastComments 仪表板的 **Connected Apps** 下，显示其拥有的权限以及上次使用时间。在此撤销后会立即断开 Zapier 的连接；任何使用该连接的 Zap 都会停止，直至重新连接。您也可以在 Zapier 的 **My Apps** 中删除该连接。