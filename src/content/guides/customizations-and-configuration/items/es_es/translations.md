[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Con FastComments, todo el texto en el widget de comentarios es personalizable.

Puedes sobrescribir una sola pieza de texto, como el botón de enviar, o todo el texto en el widget de comentarios completo.

Por defecto, el texto en el widget de comentarios se traduce según la configuración regional del usuario. Sin embargo, podemos sobrescribir el texto, si estamos seguros
de que nuestra base de usuarios está usando la misma configuración regional/idioma, por ejemplo:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Todas las traducciones personalizables se pueden encontrar <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">aquí</a> bajo la pestaña "opciones avanzadas".

Sin embargo, hay una manera más fácil, a través de la interfaz de personalización del widget. Allí, simplemente podemos encontrar el texto que se muestra en el widget de comentarios en la configuración regional EN_US, y especificar
un reemplazo.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Panel de texto personalizado con una cadena del widget seleccionada del menú desplegable y un campo de texto de reemplazo'; title='Texto personalizado' app-screenshot-end]

Todas las sobrescrituras de traducciones actualmente afectan a todas las configuraciones regionales.