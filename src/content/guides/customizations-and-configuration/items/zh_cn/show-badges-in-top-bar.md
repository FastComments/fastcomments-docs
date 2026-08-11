[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 只会在评论线程中的评论上显示用户徽章。

然而，我们可以通过在小部件自定义页面中启用此功能，将用户徽章显示在评论表单上方的姓名旁边：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='在小部件自定义页面上显示顶部栏复选框，以在评论表单上方的姓名旁边放置徽章'; title='在顶部栏显示徽章选项' app-screenshot-end]

这将在顶部栏区域将用户的徽章与其姓名并排显示，使他们在撰写评论时的成就和状态更加突出。

请注意，必须在小部件自定义 UI 中启用此功能才能生效。您可以在代码配置中可选地将 **showBadgesInTopBar** 标志设置为 false，以在服务器级别已开启时仍选择性地禁用它：

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = '禁用在顶部栏显示徽章'; code-example-end]