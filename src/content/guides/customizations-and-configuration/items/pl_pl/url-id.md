[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Podczas renderowania wątku komentarzy lub pozostawiania komentarza, FastComments musi wiedzieć, do której strony, artykułu lub produktu te komentarze należą.

Aby to zrobić, używamy czegoś, co nazywamy „URL ID”. Jest to identyfikator, taki jak ciąg znaków lub liczba, lub adres URL.

Domyślnie, jeśli nie określisz urlId, zostanie użyty adres URL strony. Weźmiemy bieżący adres URL strony i oczyścimy go, usuwając wszelkie typowe parametry marketingowe lub identyfikatory śledzenia.

W przypadku integracji zewnętrznych, takich jak WordPress, nasza wtyczka zazwyczaj użyje identyfikatora, który reprezentuje aktualnie wyświetlaną treść jako URL ID, na przykład identyfikatora artykułu/strony.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definiowanie własnego URL ID'; code-example-end]

Jedną rzeczą, do której często odwołujemy się w tym dokumencie, jest <a href="https://fastcomments.com/auth/my-account/customize-widget/new">interfejs UI dostosowywania widgetu</a>.

Ten interfejs UI może być używany do wprowadzania wielu zmian w widgetcie komentarzy bez użycia kodu.

Tworząc regułę dostosowywania, często chcemy, aby obowiązywała na wszystkich stronach naszej witryny. Jednak w niektórych przypadkach chcemy dostosować widget komentarzy na konkretnej stronie, aby zastosować niestandardowy styl lub ewentualnie uczynić komentarze na tej stronie anonimowymi. Można również, na przykład, ustawić wyświetlanie komentarzy na żywo od razu na niektórych stronach, a na innych ukrywać je pod przyciskami powiadomień.

Wszystko to jest możliwe dzięki polu wprowadzania URL ID na tej stronie, które wygląda następująco:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Pole URL ID używane do ograniczenia reguły dostosowywania do jednej strony lub do wzorca, takiego jak */blog/*'; title='Pole wprowadzania URL ID na stronie dostosowywania widgetu' app-screenshot-end]

Wartość w tym polu powinna odpowiadać parametrowi *urlId* przekazywanemu do widgetu komentarzy. Jeśli chcesz, aby Twoja reguła dostosowywania była niezależna od *urlId*, pozostaw to pole puste lub wpisz *.

Od 2023 roku pole `URL ID` w dostosowywaniu widgetu przyjmuje również wzorce! Na przykład możesz użyć `*/blog/*`, aby dodać styl specyficzny dla Twojego bloga oraz `*/store/*`, aby mieć styl specyficzny dla Twojego sklepu, przy jednoczesnym używaniu tej samej domeny.

### Pułapki

1. Jeśli Twoja strona ma parametry hash (np. example.com#page-1) – będzie to część URL ID, domyślnie.  
2. Podczas migracji, na przykład z WordPressa do Gatsby, może być konieczne przeniesienie wartości komentarzy URL ID po początkowej migracji. W takim przypadku skontaktuj się z nami.

---