[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Met FastComments kan alle tekst in de commentaarwidget worden aangepast.

U kunt een enkel stuk tekst overschrijven, zoals de verzendknop, of alle tekst in de volledige commentaarwidget.

Standaard wordt de tekst in de commentaarwidget vertaald op basis van de locale van de gebruiker. We kunnen echter de tekst overschrijven, als we er zeker van zijn dat onze gebruikersbasis dezelfde locale/taal gebruikt, bijvoorbeeld:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Aangepaste tekst'; code-example-end]

Alle aanpasbare vertalingen zijn te vinden <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">hier</a> onder het tabblad "geavanceerde opties".

Er is echter een eenvoudigere manier, via de UI voor widgetaanpassing. Daar kunnen we eenvoudig de tekst vinden die wordt weergegeven in de commentaarwidget in de EN_US locale, en een vervanging opgeven.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Aangepast tekstpaneel met een widget‑string geselecteerd uit de dropdown en een vervangend tekstveld'; title='Aangepaste tekst' app-screenshot-end]

Alle vertaaloverschrijvingen hebben momenteel invloed op alle locales.

Dit betreft de eigen tekst van de widget. Om de opmerkingen die uw bezoekers schrijven te vertalen, zie [Comment Translation](#comment-translation).