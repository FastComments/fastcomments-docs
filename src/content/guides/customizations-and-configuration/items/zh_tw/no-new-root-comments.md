[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

將 `noNewRootComments` 設為 `true` 會使小工具隱藏根回覆區域，但仍允許使用者回覆
子評論。舉例來說，您可以在頁面載入時有條件地設定此項，以僅允許部分使用者發表頂層評論。

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---