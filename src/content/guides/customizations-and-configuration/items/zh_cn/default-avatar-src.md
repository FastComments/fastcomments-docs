[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

当用户首次使用 FastComments 发表评论时，我们会尝试从 <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a> 获取他们的头像。

但是，如果未找到头像，或用户从未在其账户中设置头像，我们将显示一个静态的默认头像图像。

要指定您自己的静态头像图像，可以使用 *defaultAvatarSrc* 设置。

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = '覆盖默认头像'; code-example-end]

这也可以在不编写代码的情况下完成。在小部件自定义页面中，查看 “Default Avatar” 部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='小部件自定义页面的默认头像部分，您可以在此设置回退头像图像 URL'; title='自定义默认头像' app-screenshot-end]

请注意，为特定用户（例如使用 SSO）定义头像的内容在其单独的章节中进行说明。