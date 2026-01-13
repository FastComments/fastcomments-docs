[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Коментарите могат да бъдат заключени, така че да не могат да се оставят нови коментари или гласове, като се зададе флагът readonly на true.

Коментарите също така няма да могат да бъдат редактирани или изтрити.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на джаджата, за цял домейн или страница:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Актуализация!

От ноември 2022 г. нишките могат да бъдат заключвани или отключвани **в реално време** от администратори и модератори чрез менюто с три точки над полето за отговор.

Това ще предотврати публикуването на нови коментари, като все пак позволява гласуване и дава възможност на потребителите да изтриват своите коментари, ако желаят, докато `readonly` не позволява това. 

Това съответства на полето `isClosed` в API на `Page`.

---