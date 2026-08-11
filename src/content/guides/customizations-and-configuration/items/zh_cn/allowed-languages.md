默认情况下，FastComments 不限制用于评论的语言。  

可能需要限制社区使用的语言。  

可以在小部件自定义页面上无需编写代码进行配置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='在小部件自定义页面上用于限制评论可使用语言的允许语言选择器'; title='允许的语言' app-screenshot-end]

系统会解析评论并确定其语言，然后与允许的语言列表进行匹配。  

如果评论使用了不被允许的语言，则会显示本地化的错误信息。