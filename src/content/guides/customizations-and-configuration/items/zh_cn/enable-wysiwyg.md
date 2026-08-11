[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 的格式化功能是通过在文本周围添加可见的锚标签，如 `<b></b>` 来实现的。点击工具栏
或使用快捷键会为您完成此操作。然而，某些社区可能希望使用不带锚标签的格式化。这称为启用
WYSIWYG（所见即所得）编辑器。该编辑器看起来与默认编辑器完全相同，只是加载了一些
额外的代码，使用户能够在不使用可见锚标签的情况下加粗、下划线等其文本。

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

这也可以在不编写代码的情况下完成。在小部件自定义页面，查看“启用高级格式化”选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='已勾选“启用高级格式化”复选框以打开 WYSIWYG 编辑器的小部件自定义页面'; title='启用 WYSIWYG' app-screenshot-end]