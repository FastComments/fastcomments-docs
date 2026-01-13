[related-parameter-start name = 'noImageUploads'; type = 'boolean'; related-parameter-end]

默认情况下 FastComments 允许上传图片。可以通过将 noImageUploads 标志设置为 true 来禁用此功能。

[code-example-start config = {noImageUploads: true}; linesToHighlight = [6]; title = 'Disabling Image Uploads'; code-example-end]

此设置可在小部件自定义页面上无需编写代码即可配置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-image-uploads'; selector = '.disable-image-uploads'; title='Disabling Image Uploads' app-screenshot-end]