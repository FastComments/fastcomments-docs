[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Por defecto, FastComments renderizará el widget de comentarios en la configuración regional determinada por el sistema y el navegador del usuario.

Cuando un usuario comenta o inicia sesión, actualizamos su última configuración regional utilizada y la usamos también para enviar correos electrónicos.

Esto afecta cómo se traduce el widget de comentarios para el usuario. La configuración regional consiste en el idioma y la región del usuario, por lo que configurar la configuración regional normalmente cambiará el idioma que se muestra al usuario.

#### Via The UI

Esto se puede definir usando la interfaz de personalización del widget. Consulte la opción "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Desplegable de Configuración regional / Idioma en la página de personalización del widget usado para sobrescribir la configuración regional detectada del visitante'; title='Cambiar la configuración regional / idioma' app-screenshot-end]

#### Via Code

Esto se puede sobrescribir con una configuración regional deseada.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Definiendo manualmente la configuración regional del usuario'; code-example-end]

### Supported Languages and Locale Codes

[¡Puedes encontrar la lista completa de idiomas compatibles y los códigos de configuración regional correspondientes aquí!](/guide-supported-languages.html#supported-languages)

### SSO Note

Si estás usando SSO, quizás quieras pasar la configuración regional del usuario en el objeto de usuario, de modo que los correos electrónicos y otras cosas se localicen correctamente para él.