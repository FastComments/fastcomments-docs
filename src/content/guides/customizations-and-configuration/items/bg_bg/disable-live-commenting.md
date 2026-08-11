[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще има активирано живо коментиране.

Това означава, че всеки зрител на нишката с коментари трябва да вижда едно и също съдържание.

Например, ако се добави коментар, този коментар трябва да се покаже. Ако коментарът бъде редактиран или премахнат,
тогава тези коментари ще бъдат редактирани или премахнати за всички зрители на нишката. Същото важи за гласовете и всички модераторски действия.

Въпреки това, можем да изключим това:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте секцията "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Секция „Disable Live Commenting“ на страницата за персонализиране на уиджета, изключваща актуализации в реално време на нишката'; title='Изключване на живото коментиране' app-screenshot-end]