[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 小工具會自動在垂直方向調整大小以容納所有可見評論。分頁是透過在目前頁面末端的「檢視下一頁」
按鈕來實現，因為我們發現這種互動對大多數使用者來說是最舒服的。

然而，有些情況會偏好使用無限滾動。例如，我們在 Stream Chat 產品中使用此功能。

我們可以透過將 **enableInfiniteScrolling** 標誌設為 true 來隱藏「檢視下一頁」按鈕並改用無限滾動：

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

這還需要加入自定義 CSS。為 `.comments` 選擇器新增自定義 CSS 以啟用捲動，例如：

[inline-code-attrs-start title = '啟用無限滾動'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

完整的可運作範例如下：

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

在上述範例中我們使用 `customCSS` 屬性，但建議改為使用 Widget Configuration UI 以提升效能。請參閱自定義 CSS 文件。 [See the Custom CSS documentation.](/guide-customizations-and-configuration.html#custom-css)