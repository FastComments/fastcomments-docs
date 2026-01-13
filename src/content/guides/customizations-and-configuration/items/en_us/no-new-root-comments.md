[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Setting `noNewRootComments` to `true` will cause the widget to hide the root reply area, but still allow users to reply
to child comments. You could, for example, set this conditionally on page load to only allow some users to leave top-level comments.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---