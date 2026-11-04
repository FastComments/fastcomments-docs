[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

默认情况下，FastComments 会根据用户的系统和浏览器所确定的语言区域（locale）来渲染评论组件。

当用户发表评论或登录时，我们会更新其上次使用的语言区域，并将其用于发送电子邮件等。

这会影响评论组件为用户呈现的翻译内容。语言区域由用户的语言和地区组成，因此配置 locale 通常会改变向用户显示文本所使用的语言。

#### 通过 UI 配置

可以通过组件自定义 UI 来定义此项。请参阅 "Locale / Language" 选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='组件自定义页面上的 Locale / Language 下拉菜单，用于覆盖访客检测到的语言区域'; title='更改语言区域 / 语言' app-screenshot-end]

#### 通过代码配置

可以通过代码覆盖为所需的语言区域。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = '手动定义用户的语言区域'; code-example-end]

### 支持的语言和语言区域代码

[你可以在此处找到支持的语言及对应语言区域代码的完整列表。](/guide-supported-languages.html#supported-languages)

### SSO 说明

如果你使用 SSO，可能需要在用户对象中传入用户的语言区域，以便为其正确地本地化电子邮件及其他内容。