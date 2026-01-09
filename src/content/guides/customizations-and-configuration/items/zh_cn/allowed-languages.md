默认情况下，FastComments 不限制用于评论的语言。 

可能希望限制社区使用的语言。

这可以在不编写代码的情况下配置，在小部件自定义页面上：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

系统会解析评论并检测其语言，然后将其与允许的语言列表进行匹配。

如果评论使用的语言不在允许范围内，将显示本地化的错误消息。