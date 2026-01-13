[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pozwala użytkownikom blokować innych użytkowników. Zablokowanie użytkownika spowoduje, że jego komentarze
będą ukryte, zapobiegnie wysyłaniu powiadomień między tymi użytkownikami i tak dalej.

Może zaistnieć potrzeba wyłączenia tej funkcjonalności. Można to zrobić w następujący sposób:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Można to również zrobić bez użycia kodu, co dodatkowo umożliwia poprawną walidację po stronie serwera, za pomocą interfejsu dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---