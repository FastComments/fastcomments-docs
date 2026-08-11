[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments wyświetli etykietę „Niezweryfikowany komentarz” dla komentarzy pozostawionych przez użytkownika, który ma niezweryfikowaną sesję przeglądarki. Więcej informacji o niezweryfikowanych komentarzach znajdziesz [tutaj](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Wyłącz etykietę niezweryfikowanego komentarza'; code-example-end]

Dodatkowo tę funkcję można wykorzystać, bez pisania kodu, w interfejsie UI dostosowywania:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Strona dostosowywania widgetu z zaznaczonym polem wyboru Wyłącz etykietę niezweryfikowanego komentarza'; title='Wyłącz etykietę niezweryfikowanego komentarza' app-screenshot-end]

---