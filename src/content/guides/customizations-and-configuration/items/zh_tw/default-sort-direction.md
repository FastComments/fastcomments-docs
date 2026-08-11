[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

預設情況下，FastComments 會依照「最相關」排序方向對評論進行排序。

「最相關」排序會考慮評論發表的時間以及投票數量來進行排序。

使用者之後可以在評論小工具的 UI 中將排序方向更改為「最舊優先」或「最新優先」。

然而，我們可以將預設值改為這三種之一。例如，如果您想先顯示最舊的評論：

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = '將預設排序更改為最舊優先'; code-example-end]

我們將 **defaultSortDirection** 的值設定為「OF」以將方向設為「OF」。

若要使用「最新優先」排序方向，我們可以這樣做：

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = '將預設排序更改為最新優先'; code-example-end]

**defaultSortDirection** 的有效值如下：

- MR: 「最新」
- NF: 「最新優先」
- OF: 「最舊優先」

這也可以不使用程式碼完成。在小工具自訂頁面中，請參閱「預設排序方向」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='預設排序方向選擇器，提供最相關、最新優先和最舊優先'; title='更改預設排序方向' app-screenshot-end]

請注意，每個頁面在每種排序方向下的評論都是預先計算好的，因此所有排序方向的效能相同。