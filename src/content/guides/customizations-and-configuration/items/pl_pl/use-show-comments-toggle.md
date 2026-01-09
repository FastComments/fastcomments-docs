[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments renderuje pole wprowadzania komentarza i wątek komentarzy jednocześnie. Aby zaoszczędzić trochę pionowej przestrzeni,
ukrywa także wszelkie inne wymagane pola, dopóki nie nastąpi interakcja z widgetem.

Jednak widget komentarzy można ukryć za przyciskiem, na przykład:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Przycisk używa różnych przetłumaczonych tekstów w zależności od tego, czy komentarze są aktualnie widoczne, czy nie. Jeśli komentarze są ukryte, używa `translations.SHOW_COMMENTS_BUTTON_TEXT`. Jeśli komentarze są widoczne, używa `translations.HIDE_COMMENTS_BUTTON_TEXT`. Tłumaczenia mogą zawierać tekst `[count]`, który zostanie zastąpiony przez zlokalizowaną liczbę.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

To zostało zaprojektowane, aby zastąpić konfigurację `hideCommentsUnderCountTextFormat`.

Liczba jest aktualizowana na żywo wraz z wątkiem komentarzy. Przycisk nie jest wyświetlany, jeśli nie ma żadnych komentarzy.

Funkcję tę można włączyć bez użycia kodu, tworząc regułę dostosowywania i włączając "Kliknij, aby wyświetlić komentarze":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]