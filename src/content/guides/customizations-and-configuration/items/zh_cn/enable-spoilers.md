[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

我们可以通过将 **enableSpoilers** 标志设置为 true 来启用剧透支持：

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = '启用剧透'; code-example-end]

This can also be done without code. In the widget customization page, see the "Enable Spoilers" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='小部件自定义页面，已勾选 Enable Spoilers 复选框，以在编辑器中添加 SPOILER 按钮'; title='启用剧透' app-screenshot-end]

When text is highlighted, and the now visible `SPOILER` button is clicked, text will be masked until the user mouses over it. For dark mode we do the same thing, with different
colors that better match dark mode.

This is also compatible with the WYSIWYG editor.