[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

當使用者第一次使用 FastComments 發表評論時，我們會嘗試從 <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a> 取得他們的頭像。

不過，如果我們找不到頭像，或使用者從未在帳戶中設定過頭像，我們會顯示一個靜態的預設頭像圖片。

若要指定自訂的靜態頭像圖片，可以使用 *defaultAvatarSrc* 設定。

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

這也可以不透過程式碼完成。在外掛自訂頁面中，請參閱「預設頭像」章節。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

請注意，為特定使用者定義頭像（例如使用 SSO）有其自己的章節說明。