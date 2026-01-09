[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 的留言小工具會在大多數網站上自動偵測深色模式。

當偵測到深色模式時，FastComments 會將原本白底黑字的顯示切換為黑底白字。圖片也會相應變化。

在頁面載入時，小工具會嘗試判斷留言小工具背後頁面背景的明暗程度。這表示頁面可能是白色背景，但如果你將留言小工具放在一個黑色背景的容器內，仍應自動啟用深色模式以確保留言可讀。

然而，這個依賴判定「亮度」的偵測機制，有時可能不會在你想要的情況下啟用深色模式。若要強制啟用，請將 *hasDarkBackground* 標記設為 true，如下所示：

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]