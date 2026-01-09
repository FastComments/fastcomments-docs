---
[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Domyślnie używane są lokalizowane daty względne. Na przykład obok niedawno dodanego komentarza możesz zobaczyć "11 minut temu".

Może być konieczne lub pożądane użycie dat absolutnych, w takim przypadku należy ustawić ten parametr na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widżetu, w Opcjach zaawansowanych:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---