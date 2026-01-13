---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Por defecto, se usan fechas relativas localizadas. Por ejemplo, junto a un comentario dejado recientemente puede ver "11 minutes ago".

Puede ser necesario o deseable mantener este formato de fecha relativa, pero también mostrar la fecha completa junto a él; en ese caso debe establecer este parámetro en true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, en Opciones avanzadas. Primero tendrá que habilitar Fechas absolutas para ver esta opción en la interfaz de usuario.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---