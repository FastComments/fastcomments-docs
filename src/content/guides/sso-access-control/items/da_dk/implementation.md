#### At nævne brugere i andre grupper

Hvis to brugere tilhører to forskellige sæt af grupper, og der ikke er nogen indbyrdes overlapning, vil de ikke kunne `@mention` hinanden.

Hvis en bruger manuelt skriver en `@mention` og indsender sin kommentar, vil den forblive som almindelig tekst. Den anden bruger vil ikke blive tagget.

#### Vedligeholdelse af grupperne

`Groups` defineres ved hjælp af henholdsvis `Pages`- og `SSOUsers`-API-ressourcerne.

`Pages`-API'et kan påkaldes for at definere det sæt af grupper, der har tilladelse til at få adgang til siden. Som standard har alle grupper, og brugere der ikke\ntilhører en gruppe, adgang.

På samme måde kan `SSOUsers`-API'et påkaldes for at definere de grupper, der er tilknyttet hver bruger.

For begge ressourcer er der ingen begrænsninger for, hvornår grupperne kan indstilles eller opdateres.

Hvis man kun ønsker at begrænse brugernes mulighed for at `@mention` hinanden, behøver `Pages` ikke at blive taget i betragtning.

##### Bemærk!

Definering og opdatering af SSO-brugergrupper kræver ikke brug af API'et, og kan i stedet opdateres automatisk ved at definere
gruppe-id'er i SSO-payloadet, der sendes til kommentar-widget'en. Dog anbefales dette ikke for store lister af grupper, da brugeren\nville skulle indsende dette payload ved hver sideindlæsning.