---
默认情况下，FastComments 在发表评论时会要求提供电子邮件。电子邮件不必是有效的，但在用户点击发送给他们的链接之前，他们的评论会显示“未验证的评论”标签。

但是，我们可以取消电子邮件的必填要求。电子邮件输入字段仍会显示，但不再是必填项。

可以通过小部件自定义 UI 进行配置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='小部件自定义 UI 中的匿名评论选项，使电子邮件字段变为可选'; title='启用匿名评论' app-screenshot-end]

仍然需要提供用户名。若要也去除此步骤，您可以
[设置默认用户名](/guide-customizations-and-configuration.html#default-username) 让所有人共享，或让 FastComments
[生成唯一的用户名](/guide-customizations-and-configuration.html#auto-generate-username) 为每位访客。

---