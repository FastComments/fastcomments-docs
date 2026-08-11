[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Domyślnie odpowiedzi na komentarze najwyższego poziomu są wyświetlane.

Można to skonfigurować tak, aby użytkownik musiał kliknąć "Show Replies" na komentarzach najwyższego poziomu, aby zobaczyć odpowiedzi podrzędne.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Opcja ukrywania odpowiedzi w interfejsie dostosowywania widgetu, ukrywająca komentarze podrzędne za linkiem Show Replies'; title='Ukryj odpowiedzi' app-screenshot-end]

To ustawienie nie wpłynie na liczbę początkowo załadowanych komentarzy najwyższego poziomu. Jeśli masz jeden komentarz najwyższego poziomu i 29 odpowiedzi, przy włączonym tym ustawieniu zobaczysz:

- Zobacz komentarz najwyższego poziomu.
- Zobacz Show Replies (29) pod tym komentarzem.

Jeśli chcesz wyświetlić wszystkie komentarze najwyższego poziomu w połączeniu z tą opcją, ustaw [starting page to -1](#starting-page).