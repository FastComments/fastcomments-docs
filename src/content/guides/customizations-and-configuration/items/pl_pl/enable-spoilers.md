[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Możemy włączyć obsługę spoilerów ustawiając flagę **enableSpoilers** na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Można to również zrobić bez kodu. Na stronie dostosowywania widżetu zobacz opcję "Włącz spoilery".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Gdy tekst jest zaznaczony, a teraz widoczny przycisk `SPOILER` zostanie kliknięty, tekst zostanie zamaskowany aż do momentu, gdy użytkownik najedzie na niego kursorem. W trybie ciemnym robimy to samo, używając innych kolorów, które lepiej pasują do trybu ciemnego.

Jest to również zgodne z edytorem WYSIWYG.