[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

預設情況下，如果使用者輸入評論後，刷新頁面、關閉分頁或在提交前離開頁面，草稿會悄悄遺失。

將 **warnOnUnsavedComment** 設為 true，會在使用者離開頁面時（只要任何評論框或正在編輯的內容仍有文字）顯示瀏覽器確認對話框。提交評論後文字會被清除，因而不會顯示提示。

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = '警告未儲存的評論'; code-example-end]

此提示使用瀏覽器內建的對話框。現代瀏覽器會顯示自己的文字並忽略自訂文字，因此無法自訂訊息內容。

此選項會在需要時載入一個小型擴充套件，對於未啟用此功能的站點，對小工具本身不會產生任何額外負擔。