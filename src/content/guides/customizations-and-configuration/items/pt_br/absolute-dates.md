[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Por padrão, são usadas datas relativas localizadas. Por exemplo, ao lado de um comentário deixado recentemente você pode ver "há 11 minutos".

Pode ser necessário ou desejável usar datas absolutas, caso em que você define este parâmetro como true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Isto pode ser personalizado sem código, na página de personalização do widget, em Opções Avançadas:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]