[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

De forma predeterminada, FastComments mostrará el widget de comentarios en la locale determinada por el sistema y el navegador del usuario.

Cuando un usuario comenta o inicia sesión, actualizamos su última locale utilizada y la usamos también para el envío de correos electrónicos.

Esto afecta cómo se traduce el widget de comentarios para el usuario. Locale consiste en el idioma y la región del usuario, por lo que configurar la locale normalmente cambiará el idioma que se muestra al usuario.

#### A través de la interfaz de personalización del widget

Esto puede definirse usando la interfaz de personalización del widget. Vea la opción "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Mediante código

Esto puede sobrescribirse con la locale deseada.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Idiomas compatibles y códigos de locale

[Puede encontrar la lista completa de idiomas compatibles y los códigos de locale correspondientes aquí.](/guide-supported-languages.html#supported-languages)

### Nota sobre SSO

Si usa SSO, quizás quiera pasar la locale del usuario en el objeto de usuario, para que los correos electrónicos y otros elementos estén correctamente localizados para ese usuario.