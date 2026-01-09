[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Domyślnie używane są zlokalizowane daty względne. Na przykład obok niedawno dodanego komentarza możesz zobaczyć "11 minutes ago".

Może być konieczne lub pożądane zachowanie tego formatu daty względnej, a jednocześnie wyświetlenie pełnej daty obok niej — w takim przypadku ustaw ten parametr na true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Można to dostosować bez użycia kodu, na stronie dostosowywania widżetu, w sekcji Zaawansowane opcje. Najpierw musisz włączyć Daty absolutne, aby zobaczyć tę opcję w interfejsie użytkownika.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---