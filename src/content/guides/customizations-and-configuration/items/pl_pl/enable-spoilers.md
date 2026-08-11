---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Możemy włączyć obsługę spoilerów, ustawiając flagę **enableSpoilers** na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Można to zrobić również bez kodu. Na stronie dostosowywania widgetu, zobacz opcję "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Strona dostosowywania widgetu z zaznaczonym polem wyboru Enable Spoilers, aby dodać przycisk SPOILER do edytora'; title='Włącz Spoilery' app-screenshot-end]

Gdy tekst jest zaznaczony, a widoczny przycisk `SPOILER` zostanie kliknięty, tekst zostanie zamaskowany, dopóki użytkownik nie najedzie na niego myszką. W trybie ciemnym robimy to samo, używając innych kolorów lepiej pasujących do trybu ciemnego.

Jest to również kompatybilne z edytorem WYSIWYG.

---