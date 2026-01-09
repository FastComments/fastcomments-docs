Som standard viser FastComments brugerens navn, som de indtastede det, eller som det blev sendt til os via SSO.

Dog kan det være ønskeligt at maskere eller vise brugerens navn på en anden måde. For eksempel, hvis brugerens navn er Allen Rex, måske
du kun vil vise "Allen R.".

Dette kan gøres uden kode i Widget Customization UI, under indstillingen kaldet `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

De tilgængelige formater er:

- Capitalize (viser eksempelbrugeren som Example User)
- Last Initial (viser Example User som Example U.)
- All Initials (viser Example User som E. U.)
- Show "Anonymous"

Ændringen træder i kraft med det samme. Brugere vil stadig se deres fulde brugernavn øverst i kommentarfeltet, for dem selv, men deres kommentarer vil vise
det ændrede brugernavn.

Brugernavne maskeres på serversiden for at beskytte brugerne.