[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

当用户第一次使用 FastComments 评论时，我们会尝试从 <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a> 获取他们的头像。

但是，如果我们找不到头像，或者用户从未在其帐户中设置头像，我们将渲染一个静态默认头像图片。

要指定您自己的静态头像图片，可以使用 *defaultAvatarSrc* 设置。

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

也可以不写代码完成。在小部件自定义页面，参见 "Default Avatar" 部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

请注意，为特定用户定义头像（例如使用 SSO）在单独的章节中进行了说明。

---