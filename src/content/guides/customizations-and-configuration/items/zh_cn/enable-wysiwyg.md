---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 的格式化功能是通过在你的文本周围添加可见的锚点标签，比如 `<b></b>` 来实现的。点击工具栏
或使用快捷键会为你完成此操作。不过，有些社区可能希望选择在不使用锚点标签的情况下进行格式化。这被称为启用
WYSIWYG（所见即所得）编辑器。该编辑器的外观与默认编辑器完全相同，只是会加载一些
额外的代码，使用户能够在不使用可见锚点标签的情况下对他们的文本进行加粗、下划线等格式化。

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

这也可以不通过代码完成。在小部件自定义页面中，查看 "Enable Advanced Formatting" 选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---