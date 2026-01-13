[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

За веће пројекте прилагођавања стилова, може бити пожељно почети од нуле и уопште не користити подразумеване стилове.

Сви подразумевани стилови могу бити уклоњени подешавањем параметра **noStyles** на true, као у примеру:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Ово се може прилагодити без писања кода, на страници за прилагођавање видгета, у оквиру Напредних опција:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]