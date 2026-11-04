Som standard tillader FastComments ikke iframes i kommentarer. Når du aktiverer medie-indlejringer, kan kommentatorer indsætte embed-koden (`<iframe>`-uddraget) fra betroede udbydere som YouTube, Vimeo, SoundCloud og Spotify, og den vil blive vist inline i kommentaren.

Af sikkerhedsmæssige årsager er dette ikke et client-side widget-konfigurationsflag. Det er en server-side indstilling, der valideres, hver gang en kommentar gemmes, så den kan ikke aktiveres fra siden. Kun iframes, der peger på en indbygget liste over betroede udbydere, tillades. Alle andre iframes fjernes.

Dette gøres uden kode, på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Indstillingen for medie-indlejringer aktiveret på widget-tilpasningssiden, så kommentatorer kan indsætte betroede iframe-embeds'; title='Tillad medie-indlejringer' app-screenshot-end]

### Tilføj dine egne udbydere

Hvis du vil tillade indlejringer fra en udbyder, der ikke er på den indbyggede liste over betroede, skal du tilføje dets hostname i feltet "Additional Embed Domains" på samme side. Disse hostnames tillades ud over de indbyggede udbydere. Matchingen er eksakt, så inkluder det fulde hostname (for eksempel player.example.com). Alt, hvad du ikke angiver, forbliver blokeret.

Både den almindelige kommentarboks og WYSIWYG-editoren understøtter indsættelse af en embed. I WYSIWYG-editoren indsættes embed'en som en blok, der kan fjernes.