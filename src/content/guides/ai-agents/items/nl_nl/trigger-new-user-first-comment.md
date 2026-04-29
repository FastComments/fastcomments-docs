Vindt plaats wanneer een gebruiker zijn/haar eerste opmerking op deze site (uw tenant) plaatst. Dit gebeurt **eenmalig per gebruiker** - volgende opmerkingen van dezelfde gebruiker activeren het niet opnieuw.

### Context die de agent ontvangt

- De nieuwe opmerking.
- Optionele thread / gebruikersgeschiedenis / pagina-context zoals geconfigureerd.

Wanneer gebruikersgeschiedeniscontext aan staat, zal de lijst met recente opmerkingen van de gebruiker uiteraard leeg zijn (of alleen deze bevatten), maar de trust factor en accountleeftijd worden gevuld.

### Opmerkingen

- "Eerste opmerking op deze site" is gescoord op de **tenant**, niet site-breed over FastComments. Een gebruiker met opmerkingen op andere FastComments-sites activeert deze trigger nog steeds de eerste keer dat hij/zij op die van u post.
- De trigger wordt alleen geactiveerd voor gebruikers met een userId. Anonieme, niet-geverifieerde opmerkingen zonder een stabiele userId activeren het niet.
- De trigger wordt geactiveerd wanneer de opmerking is goedgekeurd/zichtbaar (niet op het moment van het oorspronkelijke plaatsen). Bewerking of moderatoracties op eerste opmerkingen activeren het niet opnieuw.

### Veelvoorkomende toepassingen

- **Welkomsgroet** - het [Welkomstgroet-sjabloon](#template-welcome-greeter) is gebouwd rond deze trigger.
- **Onboarding** - stuur een [DM-waarschuwing](#tool-warn-user) (hier gebruikt als een vriendelijke heads-up in plaats van een waarschuwing) die de gebruiker verwijst naar de communityrichtlijnen.
- **Melding voor beoordelaar** - als u wilt dat een mens elke nieuwe beoordelaar's eerste bericht bekijkt, kan [`mark_comment_reviewed`](#tools-overview) ze markeren voor beoordeling.