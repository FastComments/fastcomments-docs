---
[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Por defecto se usan fechas relativas localizadas. Por ejemplo, junto a un comentario publicado recientemente puede ver "hace 11 minutos".

Puede ser necesario o deseable usar fechas absolutas, en cuyo caso debe establecer este parámetro en true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, en Opciones avanzadas:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---