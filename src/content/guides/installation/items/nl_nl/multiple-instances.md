Elke instantie van de reactiewidget is geïsoleerd. Hierdoor ondersteunt FastComments van nature meer dan één instantie per pagina, of meerdere instanties die naar dezelfde chatthread verwijzen.

In het geval van de VanillaJS-bibliotheek hoeft u de reactiewidget simpelweg aan verschillende DOM-knooppunten te koppelen. Als u simpelweg de huidige thread op de pagina wilt bijwerken, zie [Reactiethreads Wisselen Zonder de Pagina te Herladen](guide-customizations-and-configuration.html#switching-comment-threads);

### Authenticatiestatus Synchroniseren Over Meerdere Instanties

Laten we het voorbeeld bekijken van een aangepaste single-page-applicatie die een lijst is van veelgestelde vragen met hun eigen reactiethread.

In dit geval hebben we meerdere instanties van FastComments tegelijk in de DOM.

Dit is prima, maar het brengt enkele uitdagingen voor de gebruikerservaring met zich mee.

Overweeg deze stroom:

1. De gebruiker bezoekt de pagina met een lijst vragen, elk met hun eigen reactiewidget.
2. De gebruiker voert zijn gebruikersnaam en e-mail in en laat een vraag achter op een van de threads.
3. Hij ziet een ander FAQ-item waarover hij een vraag heeft.
4. Hij gaat opnieuw reageren. Moet hij zijn e-mail en gebruikersnaam opnieuw invoeren?

In dit geval handelt FastComments het synchroniseren van de authenticatiestatus over widget-instanties voor u af. In stap vier zal de gebruiker al tijdelijk geauthenticeerd zijn omdat hij zijn gebruikersnaam en e-mail op dezelfde pagina heeft ingevoerd.
