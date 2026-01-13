[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

По подразбиране полето за въвеждане на коментар е **преди** нишката с коментари. Въпреки това, като зададем този конфигурационен параметър на true, можем да го преместим **след** нея.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; title='Moving The Reply Box to The Bottom' app-screenshot-end]

---