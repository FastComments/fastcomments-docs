[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

我們可以透過將 **enableSpoilers** 標記設為 true 來啟用劇透支援：

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

也可以不透過程式碼來完成。在小工具自訂頁面中，請參閱「啟用劇透」選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

當文字被選取後，點擊現已可見的 `SPOILER` 按鈕，文字會被遮罩，直到使用者將滑鼠移到其上方。對於深色模式我們採用相同的做法，使用不同的
顏色以更符合深色模式。

這也相容於 WYSIWYG 編輯器。