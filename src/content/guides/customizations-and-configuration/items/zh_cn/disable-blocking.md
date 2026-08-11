[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 允许用户阻止其他用户。阻止用户会导致其评论被屏蔽，阻止用户之间的通知等。

可能需要禁用此功能。可以这样做：

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

这也可以在不编写代码的情况下完成，通过小部件自定义 UI 同时实现正确的服务器端验证：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='在小部件自定义 UI 中的禁用阻止选项，可阻止用户相互阻止'; title='Disable Blocking' app-screenshot-end]