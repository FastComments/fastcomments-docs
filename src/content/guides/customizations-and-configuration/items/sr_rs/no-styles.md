[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

За веће пројекте прилагођеног стилизовања, можда је пожељно почети са чистим листом и уопште не користити подразумевано стилизовање.

Све подразумевано стилизовање може се уклонити постављањем параметра **noStyles** на true, на следећи начин:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Онемогућавање свих подразумеваних стилова'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета, у одељку Напредне опције:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Потврдни оквир за онемогућавање свих подразумеваних стилова омогућен у Напредним опцијама на страници за прилагођавање виџета'; title='Онемогућавање свих подразумеваних стилова' app-screenshot-end]