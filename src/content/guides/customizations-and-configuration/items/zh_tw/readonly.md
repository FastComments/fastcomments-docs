[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

可以透過將 readonly 標誌設為 true 來鎖定評論，使無法留下新的評論或投票。

評論也將無法被編輯或刪除。

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = '將評論串設為唯讀'; code-example-end]

這可以在 widget 自訂頁面上，為整個網域或頁面進行自訂，無需撰寫程式碼：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='將評論串設為唯讀' app-screenshot-end]

## 更新！

自 2022 年 11 月起，管理員和版主可以透過回覆區上方的三點選單**即時**鎖定或解鎖討論串。

這將阻止新的評論，但仍允許投票，且允許使用者刪除自己的評論（如欲），而 `readonly` 則不允許這些功能。

這對應到 `Page` API 中的 `isClosed` 欄位。