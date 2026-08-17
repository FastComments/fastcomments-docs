---
[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments ће имати омогућено живо коментарисање.

То значи да сваки гледаоца нити коментара треба да види исти садржај.

На пример, ако се дода коментар, тај коментар треба да се прикаже. Ако се коментар уреди или уклони,
онда ће се ти коментари уредити или уклонити за све гледаоце нити. Исто важи за гласове и све радње модерације.

Међутим, можемо да онемогућимо ово:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте одељак „Disable Live Commenting“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Одељак „Disable Live Commenting“ на страници за прилагођавање виџета, искључује ажурирања нити у реалном времену'; title='Онемогућено живо коментарисање' app-screenshot-end]

---