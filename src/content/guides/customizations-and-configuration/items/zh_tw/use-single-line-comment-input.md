---
[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 允許使用者輸入任意行數的評論，直至預設的字元限制。

然而，可能需要限制使用者只能輸入單行文字。某些範例使用情境包括線上競標或即時聊天，FastComments 可用於此類情況。

我們可以如下啟用 **useSingleLineCommentInput** 旗標：

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = '啟用單行評論輸入'; code-example-end]

這也可以不使用程式碼完成。在小工具自訂頁面中，請參閱「啟用單行評論輸入」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='在小工具自訂頁面中開啟的單行評論輸入核取方塊，將輸入限制為單行'; title='啟用單行評論輸入' app-screenshot-end]

請注意，每個頁面在每個排序方向的評論都是預先計算的，因此所有排序方向的效能相同。

---