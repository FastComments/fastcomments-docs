[related-parameter-start name = 'noImageUploads'; type = 'boolean'; related-parameter-end]

デフォルトでは FastComments は画像のアップロードを許可しています。この設定は noImageUploads フラグを true に設定することで無効にできます。

[code-example-start config = {noImageUploads: true}; linesToHighlight = [6]; title = '画像アップロードの無効化'; code-example-end]

コードを使用せずに、ウィジェットのカスタマイズページでカスタマイズできます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-image-uploads'; selector = '.disable-image-uploads'; alt='ウィジェットカスタマイズページ設定で画像アップロード無効化チェックボックスがオンになっている状態'; title='画像アップロードの無効化' app-screenshot-end]

---