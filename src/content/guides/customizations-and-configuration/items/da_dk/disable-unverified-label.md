[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments en etiket "Uverificeret kommentar" for kommentarer, der er efterladt til en bruger, som
har en uverificeret browser-session. Læs mere om uverificeret kommentering [her](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Derudover kan denne funktion bruges uden at skrive kode i Tilpasnings-UI'en:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---