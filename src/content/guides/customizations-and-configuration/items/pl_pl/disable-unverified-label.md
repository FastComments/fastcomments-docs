---
[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pokaże etykietę "Niezweryfikowany komentarz" dla komentarzy, które zostały zostawione dla użytkownika, który
ma niezweryfikowaną sesję przeglądarki. Więcej informacji o komentowaniu bez weryfikacji znajdziesz [tutaj](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Dodatkowo tę funkcję można zastosować bez pisania kodu w interfejsie personalizacji:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---