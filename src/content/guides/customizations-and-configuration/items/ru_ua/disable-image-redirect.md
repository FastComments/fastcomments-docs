[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователям загружать изображения. Когда пользователь нажимает на это изображение, FastComments, по умолчанию,
откроет новую вкладку, чтобы показать изображение полностью. Установка этого флага в true отключает такое поведение:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Если вы не планируете перехватывать щелчок по изображению самостоятельно (см. [onImageClicked](#callbacks)), мы рекомендуем сочетать это с некоторыми стилями
чтобы убрать видимость того, что изображение можно нажать.