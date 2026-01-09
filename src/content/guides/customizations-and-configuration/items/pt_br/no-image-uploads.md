[related-parameter-start name = 'noImageUploads'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments permite uploads de imagens. Isso pode ser desativado definindo a flag noImageUploads como true.

[code-example-start config = {noImageUploads: true}; linesToHighlight = [6]; title = 'Disabling Image Uploads'; code-example-end]

Isto pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-image-uploads'; selector = '.disable-image-uploads'; title='Disabling Image Uploads' app-screenshot-end]