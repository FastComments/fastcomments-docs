Da e-mail-skabeloner understøtter variabler og logik, er det muligt at oprette skabeloner
som fejler ved gengivelse, eller nogle gange undlader at blive gengivet.

Det kan være meget frustrerende at diagnosticere og fejlfinde, især hvis det er et intermitterende problem, eller
hvis det kun opstår, når dataene ser ud på en bestemt måde.

For at hjælpe har FastComments Email Templates et par funktioner:

1. Hvis skabelonen ikke kan forhåndsvises, kan den ikke gemmes. En fejlmeddelelse vises.
2. Gengivelsesfejl for skabeloner spores og rapporteres i admin-UI'en.

Den anden punkt beskriver gengivelsesfejl, der sker i produktion. Altså: du opretter en skabelon, som forhåndsvises
fint - men som senere fejler af en eller anden grund. For eksempel, hvis vi har dette i vores skabelon:

    <% if (comment.commenterEmail.includes('test') { %>

Dette kan nogle gange fejle, hvis vi har anonym kommentering aktiveret, eftersom e-mail ikke altid
vil være tilgængelig. Hvordan finder vi så ud af det?

Svaret er, at fejl synliggøres to steder. For det første viser selve skabelonlisten
et antal gengivelsesfejl for hver skabelon.

Når man ser en skabelon kan vi se et antal, pr. fejl, af hvor mange gange skabelonen
har fejlet ved gengivelse.

En nulstillingsknap er placeret ved siden af hver fejl og dens tæller, så vi kan nulstille tælleren
efter at have løst et problem.