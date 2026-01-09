[related-parameter-start name = 'noImageUploads'; type = 'boolean'; related-parameter-end]

デフォルトでは FastComments は画像のアップロードを許可します。これは noImageUploads フラグを true に設定することで無効にできます。

[code-example-start config = {noImageUploads: true}; linesToHighlight = [6]; title = 'Disabling Image Uploads'; code-example-end]

これはウィジェットのカスタマイズページで、コードを使わずに変更できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-image-uploads'; selector = '.disable-image-uploads'; title='Disabling Image Uploads' app-screenshot-end]

---