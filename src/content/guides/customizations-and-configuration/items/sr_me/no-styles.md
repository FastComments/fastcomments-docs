[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

За веће пројекте прилагођавања стила, може бити пожељно почети од нуле и уопште не користити подразумевани стил.

Сви подразумевани стилови могу бити уклоњени подешавањем параметра **noStyles** на true, на следећи начин:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета, у одељку Напредне опције:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]