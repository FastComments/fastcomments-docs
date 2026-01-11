Med FastComments SSO Access Control, nogle gange kaldet RBAC, kan brugere begrænses til kun at få adgang til bestemte sider eller kommentarsamtaler. Derudover kan brugere kun `@mention` hinanden i samme gruppe.

## Detaljeret

Vi kan placere `Users` og valgfrit `Pages` i grupper.

Når `Users` placeres i grupper, og `Limit Comments by SSO User Groups` er aktiveret i Widget-indstillinger, så vil brugere
kun se kommentarer fra brugere i deres samme grupper og vil kun kunne `@mention` brugere i de samme grupper.

Derudover kan `Pages` placeres i grupper, og så kan brugere kun få adgang til kommentarer for sider, de har adgang til.

Vi kalder dette "Brugerniveau"-grupper kontra "Sideniveau"-grupper.

Så hvilken passer til dig?

#### Brug Brugerniveau-grupper hvis...

- Du ønsker at bruge den samme `urlId` værdi (side-URL eller artikel-ID), men begrænse kommentarer efter gruppe.
- For eksempel ønsker du at have "New User" og "Veteran User" grupper, og de skal aldrig se hinandens kommentarer på de samme sider.

#### Brug Sideniveau-grupper hvis...

- Dine grupper har specifikke sider.
- For eksempel bør brugere i gruppen "Public Pages" aldrig kunne se artiklerne i "Top Secret" artiklerne.

---