[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

使用 FastComments，评论小部件中的所有文本都是可自定义的。

您可以覆盖单个文本，例如提交按钮，或覆盖整个评论小部件中的所有文本。

默认情况下，评论小部件中的文本会根据用户的语言环境进行翻译。不过，如果我们确信用户群使用相同的地区/语言，例如：

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = '自定义文本'; code-example-end]

所有可自定义的翻译可以在 <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">此处</a> 的“高级选项”标签下找到。

不过，有一种更简单的方法，通过小部件自定义 UI。在那里，我们可以直接找到 EN_US 语言环境下评论小部件中显示的文本，并指定替换内容。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='自定义文本面板，包含从下拉菜单中选择的小部件字符串和替换文本字段'; title='自定义文本' app-screenshot-end]

所有翻译覆盖目前会影响所有语言环境。

---