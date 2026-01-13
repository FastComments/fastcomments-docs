[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

By default, the formatting functionalities in FastComments is done by adding visible anchor tags like `<b></b>` around your text. Clicking the toolbar
or using shortcuts does this for you. However, some communities may want to opt into using formatting without anchor tags. This is called enabling the
WYSIWYG (what you see is what you get) editor. This editor looks exactly the same as the default one, except it loads some
extra code which allows users to bold, underline, etc their text without visible anchor tags.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

This can also be done without code. In the widget customization page, see the "Enable Advanced Formatting" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]
