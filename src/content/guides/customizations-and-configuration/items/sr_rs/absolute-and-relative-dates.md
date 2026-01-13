---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

По подразумеваној вредности користе се локализовани релативни датуми. На пример, поред недавно остављеног коментара можете видети „пре 11 минута“.

Може бити потребно или жељено задржати овај формат релативног датука, али истовремено приказати и пун датум уз њега — у том случају подесите овај параметар на true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање видгета, у оквиру Напредних опција. Прво ћете морати да омогућите Апсолутне датуме да бисте ову опцију видели у корисничком интерфејсу.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---