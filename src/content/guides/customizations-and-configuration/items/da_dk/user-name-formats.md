---
Som standard vil FastComments vise brugerens navn, som de indtastede det, eller hvordan det blev sendt til os via SSO.

Det kan dog være ønskeligt at maskere eller vise brugerens navn på en anden måde. For eksempel, hvis brugerens navn er Allen Rex, vil du måske kun vise "Allen R.".

Dette kan gøres uden kode i Widget‑tilpasnings‑UI’en, under indstillingen kaldet `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format dropdown åbnet med valg som Capitalize, Last Initial og All Initials'; title='Skift navneformat' app-screenshot-end]

De tilgængelige formater er:

- Capitalize (vis eksempelbruger som Example User)
- Last Initial (vis Example User som Example U.)
- All Initials (vis Example User som E. U.)
- Vis "Anonymous"

Effekten af at ændre dette er øjeblikkelig. Brugere vil stadig se deres fulde brugernavn øverst i kommentarfeltet for sig selv, men deres kommentarer vil vise det modificerede brugernavn.

Brugernavne maskeres på serveren for at beskytte brugerne.
---