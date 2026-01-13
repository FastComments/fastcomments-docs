[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 允许用户屏蔽其他用户。屏蔽用户会导致他们的评论被屏蔽、阻止用户之间的通知等。

可能希望禁用此功能。可以按如下方式进行：

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

也可以通过小部件自定义界面在不编写代码的情况下完成此操作，这还能启用适当的服务器端验证：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]