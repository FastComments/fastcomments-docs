[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

По подразумевaњу се користе локализовани релативни датуми. На пример, поред недавно остављеног коментара можете видети "пре 11 минута".

Може бити неопходно или пожељно користити апсолутне датуме, у ком случају подесите овај параметар на true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање видгета, у оквиру Напредних опција:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]