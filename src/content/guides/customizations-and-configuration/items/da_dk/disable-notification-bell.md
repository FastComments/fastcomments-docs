[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments en notifikationsklokke i øverste højre hjørne af kommentarfeltet.

Denne klokke vil blive rød og vise et antal af de notifikationer, brugeren har. Nogle eksempler på notifikationer er:

- Bruger svarede dig.
- Bruger svarede i en tråd, du har kommenteret i.
- Bruger gav din kommentar en positiv stemme.
- Bruger svarede på en side, du har abonneret på.

Notifikationsklokken giver også en mekanisme til at abonnere på en hel side.

Vi kan dog deaktivere notifikationsklokken helt:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Deaktiver notifikationsklokke'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden, se afsnittet "Deaktiver notifikationsklokke".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Widget-tilpasningsside med afkrydsningsfeltet Deaktiver notifikationsklokke markeret'; title='Deaktiver notifikationsklokke' app-screenshot-end]