[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

預設情況下，FastComments 會根據使用者的系統與瀏覽器所決定的語系來呈現評論小工具。

當使用者發表評論或登入時，我們會更新他們最後使用的語系，並將其用於發送電子郵件。

這會影響評論小工具為使用者顯示的翻譯方式。語系包含使用者的語言與地區，因此設定語系通常會改變顯示給使用者的文字語言。

#### 透過 UI

這可以透過小工具自訂 UI 來設定。請參閱「Locale / Language」選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='在小工具自訂頁面上，用於覆寫訪客偵測到的語系的 Locale / Language 下拉選單'; title='變更 Locale / Language' app-screenshot-end]

#### 透過程式碼

這可以使用想要的語系來覆寫。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = '手動定義使用者的語系'; code-example-end]

### 支援的語言與語系代碼

[您可以在此找到完整的支援語言清單及相對應的語系代碼。](/guide-supported-languages.html#supported-languages)

### SSO 注意事項

如果您使用 SSO，您可能想在使用者物件中傳遞使用者的語系，以便電子郵件和其他項目能正確地為其本地化。

---