Når brugere kommenterer eller stemmer, og de ikke er logget ind, bliver de bedt om at angive deres e-mail og brugernavn.

For nogle websteder kan det være en barriere at bede en besøgende om at finde på et unikt brugernavn, især på mobil. FastComments kan
generere et neutralt brugernavn til hver ny besøgende og forududfylde det i brugernavnsfeltet, f.eks. `BraveOtter4172`.

Den besøgende kan lade det stå som det er, eller erstatte det med et navn efter eget valg.

Dette kan aktiveres fra Customization UI under indstillingen `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Indstillingen Generate Usernames Automatically i widget-tilpasningsgrænsefladen'; title='Generér brugernavne automatisk' app-screenshot-end]

#### Sådan fungerer det

- Hvert genererede navn er unikt. Det kontrolleres mod eksisterende konti og reserveres til den besøgendes browsersession, så to besøgende ikke tilbydes det samme navn.
- Navnet genereres kun for besøgende, der ikke har et endnu. Loggede brugere, SSO-brugere og besøgende, der allerede har kommenteret, beholder deres eksisterende navn.
- Det fungerer med eller uden [anonym kommentering](/guide-customizations-and-configuration.html#allow-anon). Med anonym kommentering slået fra skal den besøgende stadig indtaste sin e-mail, men behøver ikke længere at finde på et brugernavn.
- En tilbagevendende besøgende, der indtaster en e-mail, de har brugt før, matches til deres eksisterende konto og beholder navnet på den konto.
- Hvis en [Default Username](/guide-customizations-and-configuration.html#default-username) også er angivet, har det genererede navn forrang.