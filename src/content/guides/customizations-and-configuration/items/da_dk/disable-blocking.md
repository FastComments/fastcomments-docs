[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Som standard tillader FastComments brugere at blokere andre brugere. At blokere en bruger vil få deres kommentarer til at blive maskeret, forhindre notifikationer mellem brugerne osv.

Det kan være ønskeligt at deaktivere denne funktionalitet. Det kan gøres på følgende måde:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Dette kan også gøres uden kode, hvilket også aktiverer korrekt server-side-validering, via Widget-tilpasningsgrænsefladen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---