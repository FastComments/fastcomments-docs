---
默认情况下，FastComments 不允许在评论中使用 iframe。当您启用媒体嵌入时，评论者可以粘贴来自受信任提供商（如 YouTube、Vimeo、SoundCloud 和 Spotify）的嵌入代码（`<iframe>` 代码片段），它将在评论中内联渲染。

出于安全考虑，这不是客户端小部件配置标志，而是服务器端设置，在每条评论保存时进行验证，因此无法从页面上打开。仅允许指向内置受信任提供商列表的 iframe。任何其他 iframe 都会被移除。

这可以在小部件自定义页面上无需代码完成：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='在小部件自定义页面中打开媒体嵌入设置，允许评论者粘贴受信任的 iframe 嵌入'; title='允许媒体嵌入' app-screenshot-end]

### 添加您自己的提供商

如果您想允许来自未列入内置受信任列表的提供商的嵌入，请在同一页面的 "Additional Embed Domains" 字段中添加其主机名。这些主机名将在内置提供商之外被允许。匹配是精确的，因此请包含完整的主机名（例如，player.example.com）。未列出的任何内容都将被阻止。

普通评论框和 WYSIWYG 编辑器都支持粘贴嵌入。在 WYSIWYG 编辑器中，嵌入会作为可移除的块插入。