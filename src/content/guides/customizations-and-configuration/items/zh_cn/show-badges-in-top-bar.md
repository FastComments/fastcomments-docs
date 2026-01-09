---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 仅在评论线程中的用户评论上显示用户徽章。

但是，我们可以在小部件自定义页面启用此功能，在评论表单上方用户姓名旁显示用户徽章：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

这将在顶部栏区域于用户名旁显示该用户的徽章，使他们在撰写评论时的成就和身份更加突出。

请注意，该功能必须在小部件自定义界面中启用才能生效。您也可以在代码配置中将 **showBadgesInTopBar** 标志设置为 false，以便在服务器级别启用时选择性地将其禁用：

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---