[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments permite a los usuarios subir imágenes. Cuando un usuario hace clic en esa imagen, FastComments, por defecto,
abrirá una nueva pestaña para mostrar esa imagen a tamaño completo. Establecer este indicador a true desactiva este comportamiento:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Si no tienes previsto capturar el clic en la imagen tú mismo (ver [onImageClicked](#callbacks)), recomendamos combinar esto con algún estilo
para que no parezca que la imagen se puede hacer clic.