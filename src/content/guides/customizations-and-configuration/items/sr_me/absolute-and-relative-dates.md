---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

По подразумевању се користе локализовани релативни датуми. На пример, поред недавно остављеног коментара можете видети "пре 11 минута".

Може бити потребно или жељено задржати овај формат релативног датума, али такође приказати и пуни датум поред њега; у том случају подесите овај параметар на true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета, у оквиру Напредних опција. Прво ћете морати омогућити Апсолутне датуме да бисте видјели ову опцију у корисничком интерфејсу.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---