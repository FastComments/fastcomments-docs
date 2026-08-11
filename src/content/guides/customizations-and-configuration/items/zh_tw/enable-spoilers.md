[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

我們可以透過將 **enableSpoilers** 旗標設為 true 來啟用 spoiler 支援：

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

這也可以不寫程式碼完成。在 widget 自訂頁面中，請查看「Enable Spoilers」選項。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='已勾選「Enable Spoilers」核取方塊的 widget 自訂頁面，會在編輯器中加入 SPOILER 按鈕'; title='啟用 Spoilers' app-screenshot-end]

當文字被選取，且現在可見的 `SPOILER` 按鈕被點擊時，文字會被遮蔽，直到使用者將滑鼠移到上面。對於深色模式，我們以相同方式處理，只是使用更適合深色模式的顏色。

這也相容於 WYSIWYG 編輯器。