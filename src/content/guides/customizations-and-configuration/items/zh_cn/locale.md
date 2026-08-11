[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

默认情况下，FastComments 会根据用户的系统和浏览器确定的语言环境来渲染评论小部件。

当用户发表评论或登录时，我们会更新他们最近使用的语言环境，并将其用于发送电子邮件等。

这会影响评论小部件为用户呈现的翻译内容。语言环境由用户的语言和地区组成，因此配置语言环境通常会更改向用户显示文本的语言。

#### Via The UI

这可以通过小部件自定义 UI 定义。请参阅 “语言/地区” 选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='用于覆盖访客检测到的语言环境的部件自定义页面上的语言/地区下拉菜单'; title='更改语言/地区' app-screenshot-end]

#### Via Code

这可以使用所需的语言环境进行覆盖。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = '手动定义用户的语言环境'; code-example-end]

### Supported Languages and Locale Codes

[You can find the complete list of supported languages and the corresponding locale codes here.](/guide-supported-languages.html#supported-languages)

### SSO Note

如果您使用 SSO，可能需要在用户对象中传递用户的语言环境，以便电子邮件和其他内容能够为其正确本地化。

---