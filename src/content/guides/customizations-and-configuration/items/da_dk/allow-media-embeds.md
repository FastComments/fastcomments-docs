Som standard tillader FastComments ikke iframes i kommentarer. Når du aktiverer medieindlejringer, kan kommentatorer indsætte indlejringskoden (`<iframe>`-udsnittet) fra betroede udbydere som YouTube, Vimeo, SoundCloud og Spotify, og den vil blive gengivet inline i kommentaren.

Af sikkerhedshensyn er dette ikke en konfigurationsflag på klientsiden. Det er en server-side indstilling, valideret når hver kommentar gemmes, så den ikke kan aktiveres fra siden. Kun iframes, der peger på en indbygget liste over betroede udbydere, er tilladt. Enhver anden iframe fjernes.

Dette gøres uden kode, på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Tilføjelse af dine egne udbydere

Hvis du vil tillade indlejringer fra en udbyder, som ikke er på den indbyggede liste over betroede udbydere, tilføj dens værtsnavn i feltet "Additional Embed Domains" på samme side. Disse værtsnavne tillades ud over de indbyggede udbydere. Matchningen er præcis, så medtag det fulde værtsnavn (for eksempel player.example.com). Alt, hvad du ikke angiver, forbliver blokeret.

Både den almindelige kommentarboks og WYSIWYG-editoren understøtter indsætning af en indlejr. I WYSIWYG-editoren indsættes indlejringen som en blok, der kan fjernes.