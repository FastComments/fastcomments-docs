---
[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

在擷取與呈現評論時，評論小工具需要知道要從哪一頁開始。預設情況下，它會從
第一頁開始，只呈現該頁。

如有需要，可將要呈現的確切頁面作為設定 *startingPage* 傳給評論小工具。

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

請注意，頁碼從零開始，因此上述範例會呈現第二頁。

---