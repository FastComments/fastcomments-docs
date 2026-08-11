[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Por defecto, se utilizan fechas relativas localizadas. Por ejemplo, junto a un comentario recién publicado puedes ver "hace 11 minutos".

Puede ser necesario o deseado mantener este formato de fecha relativa, pero también mostrar la fecha completa junto a ella, en cuyo caso debes establecer este parámetro en true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, bajo Opciones avanzadas. Primero deberás habilitar Fechas absolutas para ver esta opción en la interfaz.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Opciones avanzadas en la página de personalización del widget con ambas fechas absolutas y la configuración combinada de fecha relativa habilitada'; title='Usar tanto fechas absolutas como relativas' app-screenshot-end]