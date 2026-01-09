[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

По подразбиране FastComments ще показва опциите за гласуване като стрелки нагоре и надолу, позволявайки на потребителите да гласуват за или против коментар.

Възможно е обаче да се промени стилът на лентата за гласуване. Настоящите опции са по подразбиране бутоните Нагоре/Надолу или използване на механизъм за гласуване със сърце.

Използваме флага **voteStyle** по следния начин:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Препоръчваме силно да го направите без код, тъй като това също активира валидации на сървърната страна. На страницата за персонализиране на джаджата вижте секцията "Стил на гласуване".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Гласуването може да бъде деактивирано, вижте `Disable Voting` по-горе при опциите за стил.

---