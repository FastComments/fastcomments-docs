[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pozwala użytkownikowi wprowadzić komentarz o dowolnej liczbie linii, aż do domyślnego limitu znaków.

Jednak może być pożądane ograniczenie użytkownika do wprowadzania tylko jednej linii tekstu. Przykładowe przypadki użycia to licytacje online lub czat na żywo, do których FastComments może być używany.

Włączamy flagę **useSingleLineCommentInput** w następujący sposób:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Włącz wprowadzanie komentarza w jednej linii'; code-example-end]

Można to zrobić również bez kodu. Na stronie dostosowywania widgetu zobacz sekcję „Włącz wprowadzanie komentarza w jednej linii”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Pole wyboru wprowadzania komentarza w jednej linii włączone na stronie dostosowywania widgetu, ograniczające wprowadzanie do jednej linii'; title='Włącz wprowadzanie komentarza w jednej linii' app-screenshot-end]

Należy zauważyć, że komentarze na każdej stronie dla każdego kierunku sortowania są wstępnie obliczane, więc wszystkie kierunki sortowania mają taką samą wydajność.