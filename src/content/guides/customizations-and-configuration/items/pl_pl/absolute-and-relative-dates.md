[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Domyślnie używane są zlokalizowane daty względne. Na przykład, obok niedawno pozostawionego komentarza możesz zobaczyć „11 minut temu”.

Może być konieczne lub pożądane zachowanie tego formatu daty względnej, ale jednocześnie wyświetlenie pełnej daty obok niej; w takim przypadku ustaw ten parametr na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu, w sekcji Opcje zaawansowane. Najpierw musisz włączyć Daty absolutne, aby zobaczyć tę opcję w interfejsie użytkownika.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Opcje zaawansowane na stronie dostosowywania widgetu z włączonymi zarówno datami absolutnymi, jak i połączonym ustawieniem dat względnych'; title='Użyj zarówno dat absolutnych, jak i względnych' app-screenshot-end]