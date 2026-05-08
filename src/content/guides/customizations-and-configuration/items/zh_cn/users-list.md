[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 不会在页面上显示用户列表。

您可以在评论组件旁边渲染当前正在查看该页面的人员列表。该列表会在用户加入和离开时实时更新，并显示他们的姓名、头像和在线指示器。

有三种布局选项：

- `1` - 顶部：在评论上方渲染一行水平重叠的头像。
- `2` - 左侧：在组件左侧渲染带有姓名和在线点的侧栏。
- `3` - 右侧：在组件右侧渲染同样的侧栏。

设置 **usersListLocation** 标志以启用此功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

默认情况下，该列表仅显示当前在线的用户。要同时包含过去在页面上发表评论的用户（但当前未在查看页面的），请将 **usersListIncludeOffline** 设置为 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

过去的评论者将不显示绿色在线点，这样可以清楚地知道当前谁在线。

具有私有资料的用户将显示为通用头像并带有“私人资料”标签，以便在不透露身份的情况下保持计数准确。

也可以无需代码进行配置。在组件自定义页面，请参阅“用户列表位置”选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

当位置设置为除“关闭”之外的任何值时，下方会显示“包含过去的评论者”复选框：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]