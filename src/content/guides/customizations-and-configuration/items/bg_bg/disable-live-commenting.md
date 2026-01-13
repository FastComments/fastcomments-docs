[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще има включени живи коментари.

Това означава, че всеки преглеждащ нишката с коментари трябва да вижда едно и също съдържание.

Например, ако е добавен коментар, този коментар трябва да се покаже. Ако коментар бъде редактиран или премахнат,
тези коментари ще бъдат редактирани или премахнати за всички преглеждащи нишката. Същото важи за гласуванията и всички модераторски действия.

Въпреки това, можем да деактивираме това:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Това може да бъде направено и без код. На страницата за персонализиране на уиджета вижте секцията "Изключване на живи коментари".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]