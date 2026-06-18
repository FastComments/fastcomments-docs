默认情况下，FastComments 不允许在评论中使用 iframes。当您启用媒体嵌入时，评论者可以粘贴来自受信任提供商（如 ` <iframe>` 片段）YouTube、Vimeo、SoundCloud 和 Spotify 等的嵌入代码，并且它会在评论中内联渲染。

出于安全考虑，这不是一个客户端小部件的配置标志。它是一个服务器端设置，在保存每条评论时进行验证，因此无法从页面上开启。仅允许指向内置受信任提供商列表的 iframes。任何其他 iframe 都会被移除。

此设置无需代码，可在小部件自定义页面完成：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### 添加您自己的提供商

如果您想允许来自内置受信任列表中没有的提供商的嵌入，请在同一页面的“附加嵌入域（Additional Embed Domains）”字段中添加其主机名。这些主机名会在内置提供商之外被允许。匹配是精确的，因此请包含完整的主机名（例如，player.example.com）。您未列出的任何主机名将保持被屏蔽。

普通评论框和所见即所得（WYSIWYG）编辑器都支持粘贴嵌入。在 WYSIWYG 编辑器中，嵌入将作为可移除的块插入。