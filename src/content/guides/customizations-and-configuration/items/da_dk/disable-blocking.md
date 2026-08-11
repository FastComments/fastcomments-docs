[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Som standard tillader FastComments brugere at blokere andre brugere. At blokere en bruger vil få deres kommentarer til at blive maskeret, forhindre meddelelser mellem brugerne, og så videre.

Det kan være ønskeligt at deaktivere denne funktionalitet. Det kan gøres på følgende måde:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Deaktiver blokering'; code-example-end]

Dette kan også gøres uden kode, hvilket også muliggør korrekt server‑side validering, via Widget‑tilpasnings‑UI'en:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Deaktiver blokering‑mulighed i widget‑tilpasnings‑UI\'en, som forhindrer brugere i at blokere hinanden'; title='Deaktiver blokering' app-screenshot-end]