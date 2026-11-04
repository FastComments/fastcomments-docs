[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Por defecto, FastComments mostrará el widget de comentarios en la configuración regional (locale) determinada por el sistema y el navegador del usuario.

Cuando un usuario comenta o inicia sesión, actualizamos su última configuración regional utilizada y también la usamos para enviar correos electrónicos.

Esto afecta cómo se traduce el widget de comentarios para el usuario. La configuración regional consiste en el idioma y la región del usuario, por lo que configurar el locale generalmente cambiará el idioma utilizado para mostrar el texto al usuario.

#### Vía la interfaz de usuario

Esto se puede definir usando la interfaz de personalización del widget. Vea la opción "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Menú desplegable de Locale / Language en la página de personalización del widget usado para sobrescribir el locale detectado del visitante'; title='Cambiando el Locale / Idioma' app-screenshot-end]

#### Vía código

Esto se puede sobrescribir con una configuración regional deseada.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Definiendo manualmente el locale del usuario'; code-example-end]

### Idiomas y códigos de locale compatibles

[Puede encontrar la lista completa de idiomas compatibles y los códigos de locale correspondientes aquí.](/guide-supported-languages.html#supported-languages)

### Nota sobre SSO

Si está usando SSO, es posible que desee pasar el locale del usuario en el objeto de usuario, para que los correos electrónicos y otras cosas se localicen correctamente para él.