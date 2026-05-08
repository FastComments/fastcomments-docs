[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 不会在页面上显示用户列表。

您可以在评论插件旁边渲染一个当前正在查看该页面的人的列表。该列表会在用户加入和离开时实时更新，并显示他们的名称、头像和在线指示器。

有三种布局选项：

- `1` - 顶部：在评论上方渲染一排水平重叠的头像。
- `2` - 左侧：在小部件左侧渲染带有姓名和在线圆点的侧边栏。
- `3` - 右侧：在小部件右侧渲染相同的侧边栏。

设置 **usersListLocation** 标志以启用该功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

默认情况下，列表仅显示当前在线的用户。要同时包含曾在该页面发表评论的用户（但当前并未查看该页面），请将 **usersListIncludeOffline** 设置为 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

过去的评论者将不显示绿色在线点，这样可以清楚地区分当前在线的用户。

具有私人资料的用户会显示为通用头像并带有“私人资料”标签，从而在不泄露身份的情况下保持计数准确。

这也可以在不写代码的情况下配置。在小部件自定义页面，查看“Users List Location”选项。当位置设置为除“Off”之外的任何值时，会在其下方出现一个“Include past commenters”复选框。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

超过 500 名实时用户时，列表可能最多有 30 秒的延迟。

---