#### Hoe reacties verschijnen in je cursussen

Zodra de LTI-integratie is ingeschakeld en de External App is geïnstalleerd, werkt FastComments automatisch op basis van de placements die je hebt geconfigureerd:

#### Assignment View

Als de **Assignment View** placement is ingeschakeld, verschijnen reacties automatisch onder elke opdracht in de cursus. Studenten en docenten zien een geneste reactiesectie wanneer ze een opdracht bekijken — er is geen extra instelling per opdracht nodig.

Elke opdracht krijgt zijn eigen aparte reactiethread.

#### Rich Content Editor Button

Als de **Editor Button** placement is ingeschakeld, kunnen docenten FastComments insluiten in elke content die de Rich Content Editor gebruikt:

1. Bewerk een **Page**, **Quiz** of **Announcement**.
2. Klik in de werkbalk van de Rich Content Editor op de knop **FastComments**.
3. FastComments wordt automatisch in de content ingesloten.
4. Sla de pagina op.

Wanneer studenten de pagina bekijken, wordt de ingesloten FastComments-widget geladen met een reactiethread die uniek is voor die pagina.

#### Automatische SSO

Bij beide placements worden studenten automatisch aangemeld via hun Canvas-account. Namen, e-mailadressen en avatars worden gesynchroniseerd via de LTI-launch; er is geen aparte login nodig.

#### Openbare toegang blokkeren (aanbevolen)

Standaard zijn FastComments-reactiegegevens openbaar leesbaar. Iedereen die de URL van een thread of het API-endpoint kan raden, kan de reacties bekijken, zelfs buiten Canvas. Voor cursusdiscussies wil je het bekijken vrijwel zeker beperken tot alleen ingeschreven studenten.

Open je <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-aanpassingspagina</a> en maak een regel aan met **Require SSO To View Comments** ingeschakeld, en stel vervolgens het beveiligingsniveau in op **Secure SSO**, zodat threads alleen via de ondertekende LTI-launch kunnen worden geladen.

Zie [Reactiethreads beveiligen met Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige uitleg, inclusief hoe je de regel kunt beperken tot één domein of pagina.