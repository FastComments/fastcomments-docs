[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

默认情况下，FastComments 会根据用户系统和浏览器确定的区域设置来渲染评论组件。

当用户发表评论或登录时，我们会更新他们最后使用的区域设置，并在发送电子邮件时使用该设置。

这会影响评论组件如何为用户进行翻译。区域设置由用户的语言和地区组成，因此配置区域设置通常会改变向用户显示文本所使用的语言。

#### 通过 UI

可以通过组件自定义界面定义此项。参见 "语言 / 区域" 选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### 通过代码

可以通过所需的区域设置覆盖此项。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = '手动定义用户的区域设置'; code-example-end]

### 支持的语言和区域代码

[您可以在此处找到受支持语言及相应区域代码的完整列表。](/guide-supported-languages.html#supported-languages)

### SSO 说明

如果您使用 SSO，您可能需要在用户对象中传递用户的区域设置，以便电子邮件和其他内容能够为他们正确地本地化。

---