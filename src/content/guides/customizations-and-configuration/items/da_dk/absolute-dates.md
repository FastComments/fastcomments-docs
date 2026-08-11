[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Som standard bruges lokaliserede relative datoer. For eksempel kan du ved siden af en nyligt efterladt kommentar se "11 minutter siden".

Det kan være nødvendigt eller ønskeligt at bruge absolutte datoer, i så fald sætter du denne parameter til true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Brug absolutte datoer'; code-example-end]

Dette kan tilpasses uden kode på widget-tilpasningssiden under Avancerede indstillinger:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Avancerede indstillinger på widget-tilpasningssiden med den absolutte dato-tilstand slået til'; title='Brug absolutte datoer' app-screenshot-end]