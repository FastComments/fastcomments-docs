---
[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

在获取和渲染评论时，评论组件需要知道从哪一页开始。默认情况下，它从
第一页开始，只渲染该页。

如果需要，可以将要渲染的确切页码作为设置 *startingPage* 传递给评论组件。

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

请注意，页码从零开始，因此上例渲染的是第二页。

---