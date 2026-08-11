[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

可以透過將 readonly 旗標設為 true 來鎖定評論，從而不允許留下新評論或投票。

評論也將無法被編輯或刪除。

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

這可以在小工具自訂頁面上，針對整個網域或單一頁面，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='在小工具自訂頁面上防止新回覆的設定，會鎖定網域或頁面的討論串'; title='將討論串設為唯讀' app-screenshot-end]

## Update!

自 2022 年 11 月起，管理員與版主可透過回覆區上方的三點選單即時 **鎖定** 或 **解鎖** 討論串。

這將阻止新評論，同時仍允許投票，且使用者若願意仍可刪除自己的評論，而 `readonly` 則不允許這些操作。 

這對應到 `Page` API 中的 `isClosed` 欄位。