[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще покаже съобщение за успех след публикуване на коментар. Това може да бъде деактивирано по следния начин:

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = 'Disable Success Message'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; title='Disable Success Message' app-screenshot-end]