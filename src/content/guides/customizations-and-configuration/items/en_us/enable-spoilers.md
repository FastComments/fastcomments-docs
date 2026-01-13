---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

You can enable spoiler support by setting the **enableSpoilers** flag to true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

This can also be done without code. On the widget customization page, see the "Enable Spoilers" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

When text is highlighted and the now visible `SPOILER` button is clicked, the text will be masked until the user hovers over it. For dark mode, we do the same thing, using different
colors that better match dark mode.

This is also compatible with the WYSIWYG editor.

---