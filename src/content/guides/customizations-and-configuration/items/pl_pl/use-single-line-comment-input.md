[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pozwala użytkownikowi wpisać komentarz składający się z dowolnej liczby wierszy, aż do domyślnego limitu znaków.

Jednakże może być pożądane ograniczenie użytkownika do wpisania tylko jednej linii tekstu. Przykładowe zastosowania to aukcje online lub czat na żywo, do których FastComments
może być użyty.

Włączamy flagę **useSingleLineCommentInput** w następujący sposób:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Można to również zrobić bez kodu. Na stronie dostosowywania widżetu zobacz sekcję "Włącz jednolinijkowe pole komentarza".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Należy pamiętać, że komentarze na każdej stronie dla każdego kierunku sortowania są wstępnie obliczane, więc wszystkie kierunki sortowania mają taką samą wydajność.