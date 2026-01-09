---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Som standard bruges lokaliserede relative datoer. For eksempel kan du ved en nyligt efterladt kommentar se "11 minutter siden".

Det kan være nødvendigt eller ønskeligt at bevare dette relative datoformat, men også vise den fulde dato ved siden af; i så fald sætter du denne parameter til true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Dette kan tilpasses uden kode, på widgetens tilpasningsside, under Avancerede indstillinger. Du skal først aktivere Absolutte datoer for at se denne mulighed i brugergrænsefladen.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---