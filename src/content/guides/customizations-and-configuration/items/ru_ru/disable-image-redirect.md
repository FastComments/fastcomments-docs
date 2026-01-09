[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователям загружать изображения. Когда пользователь нажимает на изображение, FastComments по умолчанию откроет новую вкладку, чтобы показать это изображение в полном размере. Установка этого флага в true отключает это поведение:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Если вы не планируете обрабатывать щелчок по изображению самостоятельно (см. [onImageClicked](#callbacks)), мы рекомендуем сочетать это с применением стилей, чтобы убрать видимость того, что изображение можно нажать.

---