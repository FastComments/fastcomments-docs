[related-parameter-start name = 'disableToolbar'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 在撰写评论时会显示工具栏，以提供装饰文本和上传图片的快捷方式。

此工具栏可以通过代码或自定义 UI 来禁用。

[code-example-start config = {disableToolbar: true}; linesToHighlight = [6]; title = '禁用工具栏'; code-example-end]

这也可以在不使用代码的情况下完成。在小部件自定义页面中，查看 “Disable The Reply Toolbar” 选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-toolbar']; selector = '.disable-toolbar'; alt='已选中 “Disable The Reply Toolbar” 复选框以移除格式化快捷方式的小部件自定义页面'; title='禁用工具栏' app-screenshot-end]