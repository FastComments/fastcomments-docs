---
[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

对于较大型的自定义样式项目，通常希望从头开始，不使用任何默认样式。

可以通过将 **noStyles** 参数设置为 true 来移除所有默认样式，方法如下：

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

也可以在小部件自定义页面的“高级选项”下，无需编写代码即可进行此项设置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]

---