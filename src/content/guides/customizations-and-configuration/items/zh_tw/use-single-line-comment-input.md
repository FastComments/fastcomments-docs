[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 允許使用者輸入任意行數的評論，直到達到預設的字元上限。

然而，有時會希望限制使用者只能輸入單行文字。範例使用情境包括線上出價，或即時聊天，FastComments
可用於這些情境。

我們可如下啟用 **useSingleLineCommentInput** 標記：

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

這也可以在不使用程式碼的情況下完成。在 widget 自訂頁面中，請參閱「啟用單行評論輸入」章節。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

請注意，每一頁上針對每種排序方向的評論都是預先計算的，因此所有排序方向在效能上是一樣的。