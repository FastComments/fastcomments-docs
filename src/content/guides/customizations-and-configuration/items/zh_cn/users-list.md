[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 不会在页面上显示用户列表。

您可以在评论小部件旁边渲染当前正在查看页面的用户列表。该列表会实时更新，随着用户加入和离开，并显示他们的姓名、头像和在线指示器。

有三种布局选项：

- `1` - 顶部：在评论上方渲染的水平行重叠头像。
- `2` - 左侧：在小部件左侧渲染的带有姓名和在线点的侧边栏。
- `3` - 右侧：在小部件右侧渲染的相同侧边栏。

设置 **usersListLocation** 标志以启用此功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = '在右侧显示用户列表'; code-example-end]

默认情况下，列表仅显示当前在线的用户。若要同时包括过去在页面上发表评论但当前未在观看的用户，请将 **usersListIncludeOffline** 设置为 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = '包括过去的评论者'; code-example-end]

过去的评论者渲染时不带绿色在线点，以便清楚地显示当前在场的用户。

拥有私人资料的用户会显示通用头像和“私人资料”标签，以便在不泄露身份的情况下保持计数准确。

这也可以在无需代码的情况下进行配置。在小部件自定义页面，查看“用户列表位置”选项。当位置设置为除“关闭”之外的任何值时，会在其下方出现“包括过去的评论者”复选框。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='用户列表位置设置为右侧，下面显示“包括过去的评论者”复选框'; title='用户列表设置'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

过去 500 名实时用户，列表最多延迟 30 秒。