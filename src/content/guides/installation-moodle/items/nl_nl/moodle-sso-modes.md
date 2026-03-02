---
De plugin ondersteunt drie SSO-modi voor het integreren van Moodle-gebruikersaccounts met FastComments.

#### Beveiligde SSO (Aanbevolen)

Gebruikersgegevens worden aan de serverzijde ondertekend met HMAC-SHA256 met uw API Secret. Dit is de meest veilige optie en wordt aanbevolen voor gebruik in productie.

Met Beveiligde SSO:

- Gebruikersnamen, e-mails en avatars worden veilig doorgegeven aan FastComments.
- Moodle-sitebeheerders worden automatisch gesynchroniseerd als FastComments-moderatoren.
- Gebruikers kunnen zich niet voordoen als andere accounts.
- Vereist dat de **API Secret** is geconfigureerd in de plugin-instellingen.

#### Eenvoudige SSO

Gebruikersgegevens (naam, e-mail, avatar) worden client-side verzonden zonder cryptografische handtekening. Dit is eenvoudiger in te stellen omdat het geen API Secret vereist, maar het is minder veilig omdat gebruikersgegevens niet aan de serverzijde worden geverifieerd.

#### Geen

Geen SSO-integratie. Gebruikers plaatsen anoniem reacties of moeten afzonderlijk inloggen bij FastComments. Gebruik dit als u niet wilt dat Moodle-gebruikersaccounts aan reacties gekoppeld worden.

---