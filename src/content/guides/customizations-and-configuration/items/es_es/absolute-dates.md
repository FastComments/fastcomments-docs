[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Por defecto, se usan fechas relativas localizadas. Por ejemplo, junto a un comentario recién dejado puedes ver "hace 11 minutos".

Puede ser necesario o deseado usar fechas absolutas, en cuyo caso debes establecer este parámetro a true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Usar fechas absolutas'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, bajo Opciones avanzadas:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Opciones avanzadas en la página de personalización del widget con el interruptor de fechas absolutas activado'; title='Usar fechas absolutas' app-screenshot-end]