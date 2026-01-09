[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会同时呈现评论输入框和评论线程。为了节省垂直空间，
它还会在小部件被交互之前隐藏任何其他必填字段。

但是，评论小部件可以被隐藏在一个按钮后面，例如：

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

The button uses different translated text based on whether the comments are currently shown or not. If the comments are hidden, it uses `translations.SHOW_COMMENTS_BUTTON_TEXT`. If the
comments are shown, it uses `translations.HIDE_COMMENTS_BUTTON_TEXT`. The translations can contain the text `[count]` which will
be replaced by the localized count.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

此功能旨在替代 `hideCommentsUnderCountTextFormat` 配置。

评论数会随评论线程实时更新。 如果没有评论，则不显示该按钮。

可以通过创建自定义规则并启用“点击显示评论”来在无需编码的情况下启用此功能：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]