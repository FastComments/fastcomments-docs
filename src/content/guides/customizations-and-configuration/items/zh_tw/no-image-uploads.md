[related-parameter-start name = 'noImageUploads'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 允許上傳圖片。可以透過將 noImageUploads 旗標設為 true 來停用此功能。

[code-example-start config = {noImageUploads: true}; linesToHighlight = [6]; title = '停用圖片上傳'; code-example-end]

這可以在小工具自訂頁面上，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-image-uploads'; selector = '.disable-image-uploads'; alt='在小工具自訂頁面設定中，已啟用停用圖片上傳的核取方塊'; title='停用圖片上傳' app-screenshot-end]

---