[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

对于更大的自定义样式项目，可能希望从零开始，不使用任何默认样式。

通过将 **noStyles** 参数设置为 true 可以移除所有默认样式，如下所示：

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = '禁用所有默认样式'; code-example-end]

这可以在小部件自定义页面的高级选项下，无需编写代码即可进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='在小部件自定义页面的高级选项下启用的禁用所有默认样式复选框'; title='禁用所有默认样式' app-screenshot-end]