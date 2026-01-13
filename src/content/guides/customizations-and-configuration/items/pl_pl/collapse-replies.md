[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Domyślnie odpowiedzi na komentarze najwyższego poziomu są widoczne.

Można to skonfigurować tak, aby użytkownik musiał kliknąć "Pokaż odpowiedzi" przy komentarzach najwyższego poziomu, aby zobaczyć odpowiedzi podrzędne.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

To można dostosować bez użycia kodu, na stronie personalizacji widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

To ustawienie nie wpłynie na liczbę początkowo załadowanych komentarzy najwyższego poziomu. Jeśli masz jeden komentarz najwyższego poziomu i 29 odpowiedzi, po włączeniu tego ustawienia zobaczysz:

- Zobacz komentarz najwyższego poziomu.
- Zobacz "Pokaż odpowiedzi (29)" pod tym komentarzem.

Jeśli chcesz wyświetlić wszystkie komentarze najwyższego poziomu w połączeniu z tą opcją, ustaw [stronę początkową na -1](#starting-page).