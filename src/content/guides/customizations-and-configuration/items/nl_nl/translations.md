[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Met FastComments is alle tekst in de reactie-widget aanpasbaar.

U kunt één tekstonderdeel overschrijven, zoals de verzendknop, of alle tekst in de hele reactie-widget.

Standaard wordt de tekst in de reactie-widget vertaald op basis van de locale van de gebruiker. We kunnen de tekst echter overschrijven als we zeker weten
dat onze gebruikersbasis dezelfde locale/taal gebruikt, bijvoorbeeld:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Alle aanpasbare vertalingen zijn te vinden <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">hier</a> onder het tabblad "geavanceerde opties".

Er is echter een gemakkelijkere manier via de widget-aanpassingsinterface. Daar kunnen we eenvoudig de tekst vinden die in de reactie-widget wordt weergegeven in de EN_US-locale, en
een vervanging opgeven.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Alle overschrijvingen van vertalingen gelden momenteel voor alle locales.