Een subset van moderatieacties kan rechtstreeks vanuit de commentaartread zelf worden uitgevoerd, zonder naar de Comment Moderation-pagina te hoeven gaan.

Wanneer u bent ingelogd, klikt u op de bewerkknop rechtsboven in een opmerking. Als moderator zou u de volgende opties moeten hebben:

- **Vastpinnen** die opmerking
- **Verwijder** die opmerking
- **Verwijder** die opmerking + **Verbied de gebruiker** (permanent of shadow, meer details later)
- **Bewerk** die opmerking
- **Vergrendel** of **Ontgrendel** die opmerking (meer details hieronder)
- Markeer die opmerking als **Goedgekeurd** (tonen) of **Niet Goedgekeurd** (verbergen)
- Markeer die opmerking als **Spam** of **Geen Spam**

### Een opmerking vergrendelen

Het vergrendelen van een individuele opmerking voorkomt nieuwe antwoorden daarop, en voorkomt ook dat de opmerking zelf bewerkt of verwijderd kan worden totdat deze wordt ontgrendeld. Dit geldt voor iedereen, inclusief admins en moderators. Als u een vergrendelde opmerking moet bewerken of verwijderen, ontgrendel deze dan eerst, voer de wijziging uit en vergrendel hem opnieuw indien gewenst.

Een hangslotpictogram verschijnt rechtsboven in een vergrendelde opmerking zodat lezers in één oogopslag kunnen zien dat de thread gesloten is. De menu-items Bewerken en Verwijderen worden verborgen voor vergrendelde opmerkingen in zowel de comment-widget als de publieke API (`PATCH` en `DELETE` return `code: 'locked'` als ze worden aangeroepen voor een vergrendelde opmerking).

Twee opzettelijke uitzonderingen omzeilen de vergrendeling, omdat ze anders verweesde gegevens zouden achterlaten: wanneer een gebruiker zijn volledige account verwijdert (hun opmerkingen worden opgeschoond ongeacht de vergrendelingsstatus), en wanneer een moderator een gebruiker verbant met de optie "verwijder alle opmerkingen van deze gebruiker" (de sweep maakt vergrendelingen ongedaan).

### Reactiedraden sluiten

Moderators en administrators kunnen comment threads vergrendelen, of sluiten, door `Close Thread` te selecteren in het drie-puntjesmenu boven in het commentaargedeelte, als ze zijn ingelogd. Ze kunnen later op elk moment `Re-Open Thread` selecteren om het reageren weer mogelijk te maken.

Het sluiten van een comment thread voorkomt nieuwe opmerkingen, maar staat stemmen en het verwijderen van eigen opmerkingen door gebruikers nog wel toe.

Het sluiten en opnieuw openen van comment threads heeft direct effect voor alle gebruikers die de thread bekijken.

U kunt een thread ook alleen-lezen maken, waarbij de opties om te stemmen en te verwijderen worden verwijderd, door een aanpassingsregel speciaal voor die pagina te maken.

### Live bijgewerkt

Al deze acties werken de commentthreads van andere gebruikers meteen bij zonder dat zij de pagina hoeven te herladen. Moderatoracties zoals het verbergen van een opmerking of het markeren als spam verwijderen de opmerking echter niet van het scherm van **de moderator**, zodat ze indien nodig snel de actie ongedaan kunnen maken. Om aan te geven dat een opmerking verborgen is, wordt deze in vergelijking met de andere opmerkingen gemarkeerd (de kleur van de markering hangt af van de reden voor verwijdering).

For example, given users `A (commenter)`, `B (Moderator 1)`, and `C (Moderator 2)`.

...and the following scenario:

1. `User B (Moderator 1)` verbergt een opmerking.
2. Voor `User A (commenter)` wordt die opmerking onmiddellijk verborgen.
3. Voor `User C (Moderator 2)` wordt die opmerking onmiddellijk verborgen.
4. Voor de gebruiker die de wijziging heeft aangebracht, `User B (Moderator 1)`, blijft de opmerking op hun scherm staan, maar wordt gemarkeerd als verwijderd. Zij hebben de mogelijkheid om hun actie ongedaan te maken, in welk geval de andere gebruikers de update weer live zullen zien.

---