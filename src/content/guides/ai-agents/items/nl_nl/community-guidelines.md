Het veld **Communityrichtlijnen** op het bewerkformulier is een optioneel beleids-tekstblok dat bij elke uitvoering in het gebruikersrol-contextbericht voor deze agent wordt opgenomen. Het is afgebakend als onbetrouwbare tekst (dezelfde afbakening die het platform toepast op commentaarteksten en andere door gebruikers aangeleverde inhoud), dus het model behandelt het als beleidsreferentie, niet als systeeminstructies. Het is de canonieke plaats om op te schrijven "welk gedrag op deze site is toegestaan en welk niet" zodat de agent het consistent toepast.

### Hoe het verschilt van de initiële prompt

- **Initiële prompt** - de rol van de agent en de manier van beslissen. "Je bent een moderator. Geef bij voorkeur een waarschuwing boven een ban."
- **Communityrichtlijnen** - de regels van je community, in beleidsformulering. "Geen persoonlijke aanvallen. Geen promotionele links van accounts jonger dan 24 uur. Off-topic opmerkingen kunnen worden verwijderd als een thread verhitte reacties heeft."

Beide stromen in hetzelfde contextvenster, maar ze komen binnen op verschillende lagen - de initiële prompt maakt deel uit van de system-rol, het richtlijndocument is afgebakende tekst binnen het gebruikersrol-contextbericht. Die scheiding maakt het makkelijker om één van de twee bij te werken zonder de andere helemaal te hoeven herlezen.

### Wat is een goed richtlijnendocument

Een kort, specifiek, door een mens geschreven document. Lijsten werken beter dan proza:

[inline-code-attrs-start title = 'Voorbeeld communityrichtlijnen'; type='text' inline-code-attrs-end]
[inline-code-start]
Toegestaan:
- Inhoudelijk oneens zijn, ook al is het krachtig geformuleerd.
- Links naar originele bronnen, ook van nieuwe accounts.
- Off-topic opmerkingen als de bovenliggende discussie ze toestaat.

Niet toegestaan:
- Persoonlijke aanvallen tegen specifiek genoemde gebruikers.
- Doxxing of het delen van privé-informatie.
- Gecoördineerde promotieactiviteiten (meerdere opmerkingen die dezelfde externe link promoten).
- Opmerkingen die enkel bedoeld zijn om de discussie te ontsporen.

Grensgevallen:
- Hevige taal zonder doelwit. Toegestaan als het niet op een persoon is gericht.
- Politieke onderwerpen buiten het onderwerp van de pagina. Off-topic; eerst waarschuwen, niet verwijderen tenzij het aanhoudt.
[inline-code-end]

De agent past dit bij elke uitvoering toe. Als je de richtlijnen wijzigt, treedt de wijziging in werking bij de volgende trigger - eerdere uitvoeringen worden niet achteraf opnieuw geëvalueerd.

### Wat je hier niet moet zetten

- **Instructies voor uitvoeropmaak** ("antwoord in HTML", "gebruik emoji"). Die horen thuis in de [initial prompt](#personality-prompt).
- **Gelokaliseerde tekst.** Het richtlijndocument, net als de prompt, is **alleen in het Engels** om dezelfde reden - machinale vertaling kan het gedrag van de agent stilletjes veranderen. Als je beleidsregels hebt die per regio verschillen, schrijf ze dan allemaal in het Engels in dit ene document en structureer het document als "voor Duitstalige pagina's: ..."
- **Lange citaten van externe beleidsregels.** Parafraseer. Lange context kost tokens bij elke uitvoering.
- **PII of geheimen.** Deze tekst wordt bij elke uitvoering naar de LLM-provider gestuurd.

### Lengte

Het veld is beperkt tot **4000 tekens** (afgedwongen zowel door het formulier als de opslaan-route). Het tokenverbruik per uitvoering is evenredig met de lengte, dus zelfs binnen de limiet zijn een paar honderd woorden meestal voldoende. Als je communitybeleid meerdere pagina's beslaat, vat dan de delen samen die de agent nodig heeft tot een specificatie speciaal voor dit veld.

### Versiebeheer

Er is geen ingebouwde versiegeschiedenis voor het richtlijndocument - de laatst opgeslagen waarde is wat de agent gebruikt. Als je een geschiedenis wilt, kopieer het document in je eigen registratiesysteem voordat je een grote wijziging aanbrengt. De [Refine Prompts](#refining-prompts) flow kan wijzigingen in de *initial prompt* registreren, maar versieert het richtlijndocument niet.

---