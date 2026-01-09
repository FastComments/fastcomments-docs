[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По подразумјевању, FastComments ће имати омогућено уживо коментарисање.

То значи да сваки посматрач нити коментара треба да види исти садржај.

На примјер, ако се дода коментар, тај коментар ће се приказати. Ако је коментар уређен или уклоњен,
онда ће ти коментари бити уређени или уклоњени за све посматраче нити. Исто важи за гласове и све модерацијске акције.

Међутим, ово можемо онемогућити:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање видгета, погледајте секцију "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---