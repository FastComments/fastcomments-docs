#### Hoe reacties in uw cursussen verschijnen

Zodra de LTI-integratie is ingeschakeld en de External App is geïnstalleerd, werkt FastComments automatisch op basis van de placements die u hebt geconfigureerd:

#### Assignment View

If the **Assignment View** placement is enabled, comments appear automatically below every assignment in the course. Students and instructors see a threaded comment section when they view an assignment — no extra setup is needed per assignment.

Elke opdracht krijgt een eigen reactiedraad.

#### Rich Content Editor Button

If the **Editor Button** placement is enabled, instructors can embed FastComments into any content that uses the Rich Content Editor:

1. Bewerk een **Page**, **Quiz**, of **Announcement**.
2. In de werkbalk van de Rich Content Editor, klik op de **FastComments**-knop.
3. FastComments wordt automatisch in de inhoud ingesloten.
4. Sla de pagina op.

Wanneer studenten de pagina bekijken, wordt de ingesloten FastComments-widget geladen met een reactiedraad die uniek is voor die pagina.

#### Automatic SSO

In beide placements worden studenten automatisch ingelogd via hun Canvas-account. Namen, e-mails en avatars worden gesynchroniseerd via de LTI-launch; een aparte aanmelding is niet nodig.

#### Lock Down Public Access (Recommended)

Standaard zijn FastComments-reactiegegevens publiekelijk leesbaar. Iedereen die de URL of het API-endpoint van een draad kan raden, kan de reacties bekijken, zelfs buiten Canvas. Voor cursusdiscussies wilt u vrijwel zeker het bekijken beperken tot alleen ingeschreven studenten.

Open uw <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina voor widgetaanpassing</a> en maak een regel aan met **Require SSO To View Comments** ingeschakeld, en stel vervolgens het beveiligingsniveau in op **Secure SSO** zodat reactiedraden alleen kunnen worden geladen via de ondertekende LTI-launch.

Zie [Bescherming van reactiedraden met Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige handleiding, inclusief hoe de regel te beperken tot één domein of pagina.