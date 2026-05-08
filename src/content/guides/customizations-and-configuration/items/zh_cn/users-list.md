[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 不会在页面上显示用户列表。

您可以在评论小部件旁渲染当前正在查看该页面的用户列表。该列表会在用户加入和离开时实时更新，并显示他们的姓名、头像和在线指示器。

有三种布局选项：

- `1` - Top：在评论上方渲染的一排水平重叠头像。
- `2` - Left：在小部件左侧渲染的带有姓名和在线点的侧栏。
- `3` - Right：在小部件右侧渲染的相同侧栏。

设置 **usersListLocation** 标志以启用该功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

默认情况下，列表仅显示当前在线的用户。要同时包含过去在该页面发表评论但当前未在查看的用户，请将 **usersListIncludeOffline** 设置为 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

过去的评论者不会显示绿色在线点，因此可以清楚地区分当前在线的用户。

具有私人资料的用户会显示通用头像并带有“私人资料”标签，这样可以在不泄露身份的情况下保持计数准确。

这也可以通过无需编码的方式配置。在小部件自定义页面，查看“Users List Location”选项。当位置设置为非“Off”时，下方会出现一个“Include past commenters”复选框。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---