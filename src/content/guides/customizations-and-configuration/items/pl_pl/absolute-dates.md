[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Domyślnie używane są zlokalizowane względne daty. Na przykład, obok niedawno pozostawionego komentarza możesz zobaczyć "11 minut temu".

Może być konieczne lub pożądane użycie dat absolutnych, w takim przypadku ustaw ten parametr na true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Użyj dat absolutnych'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu, w sekcji Zaawansowane opcje:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Zaawansowane opcje na stronie dostosowywania widgetu z włączonym przełącznikiem dat absolutnych'; title='Użyj dat absolutnych' app-screenshot-end]

---