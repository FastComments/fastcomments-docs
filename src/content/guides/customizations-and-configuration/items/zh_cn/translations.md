[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

使用 FastComments，评论小部件中的所有文本均可自定义。

您可以覆盖单个文本，例如提交按钮，或覆盖整个评论小部件中的所有文本。

默认情况下，评论小部件中的文本会根据用户的区域设置进行翻译。然而，如果我们有信心
我们的用户群使用相同的本地/语言，例如，我们可以覆盖该文本：

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

所有可自定义的翻译可在 <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">此处</a> 的 "高级选项" 选项卡下找到。

但是，有一种更简单的方法，通过小部件自定义 UI。在那里，我们可以简单地找到在 EN_US 区域设置中评论小部件显示的文本，并指定
一个替换文本。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

目前所有翻译覆盖会影响所有区域设置。