[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

當使用者首次使用 FastComments 發表評論時，我們會嘗試從 <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a> 取得他們的頭像。

然而，如果找不到頭像，或使用者從未在其帳號中設定頭像，我們會顯示一個靜態的預設頭像圖像。

若要指定您自己的靜態頭像圖像，可使用 *defaultAvatarSrc* 設定。

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

這也可以不使用程式碼完成。在小工具自訂頁面中，請參閱「Default Avatar」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='小工具自訂頁面的預設頭像區段，您可以在此設定備用頭像圖像的 URL'; title='自訂預設頭像' app-screenshot-end]

請注意，為特定使用者（例如使用 SSO）定義頭像的說明已在其專屬章節中說明。