[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

通过将 readonly 标志设为 true，可以锁定评论，从而不允许留下新评论或投票。

评论也将无法被编辑或删除。

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = '将评论线程设为只读'; code-example-end]

这可以在无需代码的情况下，通过小部件自定义页面为整个域或单个页面进行设置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='在小部件自定义页面上阻止新回复的设置，可为域或页面锁定线程'; title='将评论线程设为只读' app-screenshot-end]

## Update!

自 2022 年 11 月起，管理员和版主可以通过回复区域上方的三点菜单**实时**锁定或解锁线程。

这将阻止新评论，同时仍然允许投票，并允许用户在需要时删除自己的评论，而 `readonly` 则不允许这些操作。 

这对应于 `Page` API 中的 `isClosed` 字段。