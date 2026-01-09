[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

По подразумевању се користе локализовани релативни датуми. На примјер, поред недавно остављеног коментара можете видјети "прије 11 минута".

Може бити потребно или пожељно користити апсолутне датуме, у том случају поставите овај параметар на true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета, у оквиру Напредне опције:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]