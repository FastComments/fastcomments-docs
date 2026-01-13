---
对于您可能不希望设为 `Administrators` 的开发人员，考虑创建一个具有以下权限的 `Administrator` 用户：

1. **Analytics Admin**
2. **Customizations Admin**
3. **Data Management Admin**
4. **Comment Moderation Admin**
5. **API/SSO Admin**

这组权限将为开发人员提供设置 FastComments 所需的一切，以及确保系统正常运行所需的可见性。

这些权限选择的理由如下：

1. **Analytics Admin**: 这可用于调试 FastComments 的使用情况。
2. **Customizations Admin**: 需要此权限以对评论小部件应用自定义样式。
3. **Data Management Admin**: 需要此权限来管理导入和导出，并设置 webhooks。
4. **Comment Moderation Admin**: 需要此权限才能查看评论数据，至少在设置期间需要。
5. **API/SSO Admin**: 这将允许他们直接从我们的平台获取 API keys。我们认为这比让一个 `Administrator` 为他们复制并通过电子邮件发送 API Secret 更安全，后者可能不太安全。

---