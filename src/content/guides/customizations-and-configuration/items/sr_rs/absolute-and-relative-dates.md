[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Подразумевано се користе локализовани релативни датуми. На пример, поред недавно остављеног коментара можете видети "пре 11 минута".

Може бити потребно или жељено да се задржи овај релативни формат датума, али исто тако прикаже пуни датум поред њега, у ком случају постављате овај параметар на true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Користи и апсолутне и релативне датуме'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета, у одељку Напредне опције. Прво ћете морати да омогућите Апсолутне датуме да бисте видели ову опцију у корисничком интерфејсу.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Напредне опције на страници за прилагођавање виџета са омогућеним и апсолутним датумима и комбинираним подешавањем релативног датума'; title='Користи и апсолутне и релативне датуме' app-screenshot-end]