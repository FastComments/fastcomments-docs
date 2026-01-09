[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

預設情況下，FastComments 會以使用者系統與瀏覽器所決定的語系來呈現評論元件。

當使用者留言或登入時，我們會更新他們上次使用的語系，並用該語系來發送電子郵件。

這會影響評論元件對使用者的翻譯顯示。語系由使用者的語言與地區組成，因此設定語系將
通常改變顯示給使用者的語言。

#### Via The UI

可以使用元件自訂介面來定義。請參閱「語系 / 語言」選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

可以以想要的語系來覆寫。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### 支援的語言與語系代碼

[您可以在此處找到完整的支援語言清單與對應的語系代碼。](/guide-supported-languages.html#supported-languages)

### SSO 注意事項

如果您正在使用 SSO，您可能會想在 user 物件中傳遞使用者的語系，這樣電子郵件與其他內容才能為他們正確地在地化。

---