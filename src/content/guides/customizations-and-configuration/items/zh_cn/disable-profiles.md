[related-parameter-start name = 'disableProfiles'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会在您点击用户头像时显示该用户的个人资料。

但是，我们可以禁用此功能：

[code-example-start config = {disableProfiles: true}; linesToHighlight = [6]; title = '禁用个人资料'; code-example-end]

这也可以在不编写代码的情况下完成。在小部件自定义页面，查看“禁用个人资料”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profiles']; selector = '.disable-profiles'; alt='已选中“禁用个人资料”复选框的小部件自定义页面，头像不再打开个人资料'; title='禁用个人资料' app-screenshot-end]

---