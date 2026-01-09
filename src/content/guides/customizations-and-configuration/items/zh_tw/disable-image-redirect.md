[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 允許使用者上傳圖片。當使用者點擊該圖片時，FastComments 預設會，
在新分頁中開啟並完整顯示該圖片。將此旗標設為 true 可停用此行為：

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

如果您不打算自行攔截圖片點擊事件（參見 [onImageClicked](#callbacks)），我們建議將此設定與一些樣式結合使用
以移除圖片可被點擊的外觀。