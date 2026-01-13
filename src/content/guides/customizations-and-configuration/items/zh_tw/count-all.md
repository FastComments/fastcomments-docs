[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

評論小工具頂部顯示的評論數可以只顯示所有「頂層」評論，也就是那些回覆
是直接回覆頁面或文章本身的，或者它可以是 **all** 巢狀評論的計數。

預設情況下，這個設定為 `true` - 它是後者的計數 - 全部評論。在舊版本的評論小工具中，
預設值為 `false`。

我們可以更改此行為，透過將 **countAll** 標記設為 true，讓它計算 **所有** 巢狀評論。

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

如果我們希望計數只反映頂層評論，則將標記設為 false。

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

目前無法在不更改程式碼的情況下自訂此行為。

---