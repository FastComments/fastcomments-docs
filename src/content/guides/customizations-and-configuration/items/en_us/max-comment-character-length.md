[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

The maximum number of characters allowed to be entered into the comment input field can be limited by the **maxCommentCharacterLength** parameter.

The default is 2000.

Elements such as image URLs are not included in the length calculation.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---