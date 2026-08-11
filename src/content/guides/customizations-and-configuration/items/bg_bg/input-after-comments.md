---
[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

По подразбиране областта за въвеждане на коментари е **преди** нишката с коментари. Въпреки това, като зададем този конфигурационен параметър на true, можем да я преместим **след**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; alt='Опция на страницата за персонализиране на уиджета, която поставя зоната за въвеждане на коментари след нишката с коментари вместо преди нея'; title='Преместване на полето за отговор към дъното' app-screenshot-end]

---