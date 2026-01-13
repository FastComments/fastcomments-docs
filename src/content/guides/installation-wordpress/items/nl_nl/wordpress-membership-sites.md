FastComments werkt met sites die alleen toegankelijk zijn voor leden door gebruik te maken van wat SSO wordt genoemd, of single-sign-on. Uw leden melden zich aan op uw WordPress-site, maar
hoeven zich geen zorgen te maken over het aanmaken van een account bij FastComments, of inloggen met sociale media, om te kunnen reageren. Als uw leden (inclusief beheerders) zijn ingelogd op
uw WordPress-site, kunnen ze reageren. Uw beheerders en moderatoren kunnen reacties ook direct vanuit uw WordPress-blogposts modereren.

<sup>(Optioneel)</sup> Denk er ook aan uw beheerders toe te voegen aan [Gebruikers & Beheerders](https://fastcomments.com/auth/my-account/users) en moderatoren aan [Moderatoren voor reacties](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
om hun ervaring te verbeteren en statistiektracking voor moderatoren mogelijk te maken.

SSO kan worden ingeschakeld door naar het plug-in-dashboard te gaan en te klikken op "SSO-instellingen".

U moet eerst de functie "Iedereen kan zich registreren" van uw site inschakelen.

Alle gebruikersinformatie wordt naadloos en veilig overgedragen aan FastComments telkens wanneer een gebruiker een discussiedraad bekijkt via [HMAC](https://en.wikipedia.org/wiki/HMAC).

Er is geen initiële of doorlopende synchronisatie die uitgevoerd hoeft te worden om uw leden naar de servers van FastComments te kopiëren. Dit gebeurt automatisch wanneer ze discussiedraden bekijken door een blogbericht te openen.

## Namen en avatars

De plugin zal automatisch de weergavenaam en avatar van de gebruiker bijwerken op al hun reacties binnen FastComments wanneer ze
een discussiedraad bekijken. Avatars worden ondersteund via Gravatar of een avatar-beheerplugin binnen WordPress omdat de plugin `get_avatar_url` zal aanroepen.