[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments ће имати омогућено уживо коментарисање.

То значи да ће сваки посматрач нити коментара видети исти садржај.

На пример, ако се дода коментар, тај коментар ће се појавити. Ако се коментар уреди или уклони,
тада ће ти коментари бити уређени или уклоњени за све посматраче нити. Исто важи за гласове и за све радње модерације.

Међутим, ово можемо онемогућити:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Ово се такође може урадити и без кода. На страници за прилагођавање виџета, погледајте одељак "Онемогући уживо коментарисање".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---