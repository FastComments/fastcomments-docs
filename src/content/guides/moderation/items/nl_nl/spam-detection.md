Standaard wordt FastComments geleverd met trainbare spamdetectie.

Terwijl u reacties modereert en deze markeert als **Spam**, of automatisch als **Spam** aangetroffen reacties markeert als **Not Spam**, leert het spam
detectiesysteem zal van deze acties leren om nauwkeuriger te bepalen wat u als spam beschouwt.

Reacties die als **Spam** zijn gemarkeerd worden niet automatisch goedgekeurd, dus ze worden niet weergegeven totdat ze expliciet als **Not Spam** worden gemarkeerd.

Spam Detection kan worden uitgeschakeld via de pagina Comment Moderation Settings.

### Verschillende spamdetectoren

FastComments ondersteunt drie manieren om spam te detecteren:

1. Een traditionele Naïve-Bayes-classificator die continu wordt getraind en die gedeeld wordt tussen alle FastComments.com tenants.
2. Een traditionele Naïve-Bayes-classificator die continu wordt getraind en die **geïsoleerd** is voor uw tenant.
3. Gebruik van ChatGPT 4.

Iedereen heeft toegang tot de gedeelde en geïsoleerde Naïve-Bayes-classificators.

De ChatGPT 4-optie is selecteerbaar op de pagina Comment Moderation Settings als u op Flex billing zit, aangezien de kosten worden berekend op basis
van gebruikte tokens.

### Vertrouwensfactor

FastComments past het spamfilter voor een gebruiker aan op basis van hoe vertrouwd die persoon is voor de betreffende site.

Bijvoorbeeld, als beheerders veel van hun reacties hebben vastgezet, dan zijn ze waarschijnlijk een zeer betrouwbare gebruiker. Of, als
ze al lange tijd lid van de site zijn en veel reacties hebben, kan hun vertrouwensfactor ook hoog zijn.

### SSO

Reacties geplaatst door SSO-gebruikers kunnen als spam worden beschouwd en zullen als zodanig worden gecontroleerd. De uitzondering is als de SSO-gebruiker
hetzelfde e-mailadres heeft als een tenant-gebruiker die een of meer van de volgende permissies heeft:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO-gebruikers met deze permissies worden niet op spam gecontroleerd voor hun reacties.

### Herhaalde berichten

FastComments zal herhaalde berichten detecteren en voorkomen. Het zal ook herhaalde berichten detecteren die erg op elkaar lijken om spam te helpen voorkomen. Dit kan
niet worden uitgeschakeld omdat het voorkomt dat ons platform voor misbruik wordt gebruikt. Als u een hoge vertrouwensfactor heeft, wordt hier rekening mee gehouden bij het voorkomen van herhaalde berichten.