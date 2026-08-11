[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Por padrão, datas relativas localizadas são usadas. Por exemplo, ao lado de um comentário recém‑deixado, você pode ver "11 minutos atrás".

Pode ser necessário ou desejado usar datas absolutas, caso em que você define este parâmetro como true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Usar datas absolutas'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget, em Opções avançadas:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Opções avançadas na página de personalização do widget com a alternância de datas absolutas ativada'; title='Usar datas absolutas' app-screenshot-end]