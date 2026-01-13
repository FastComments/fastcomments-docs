[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

可以通过将 readonly 标志设置为 true 来锁定评论，从而无法留下新的评论或投票。

评论也将无法被编辑或删除。

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

这可以在小部件自定义页面上，为整个域或某个页面在无需编码的情况下进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## 更新！

自 2022 年 11 月起，管理员和版主可以通过回复区域上方的三点菜单**实时**锁定或解锁讨论线程。

这将阻止新增评论，但仍允许投票，并允许用户在需要时删除自己的评论，而 `readonly` 不允许这些操作。

这对应于 `Page` API 中的 `isClosed` 字段。

---