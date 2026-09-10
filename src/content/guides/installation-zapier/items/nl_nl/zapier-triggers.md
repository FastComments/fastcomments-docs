## Triggers

Triggers starten een Zap wanneer er iets gebeurt in FastComments. Alle drie zijn onmiddellijk: FastComments levert het evenement aan Zapier via een webhook op het moment dat het gebeurt. Er wordt niets gepolld op uw account en er worden geen API‑credits besteed aan wachten.

| Trigger | Wordt geactiveerd wanneer |
|---------|---------------------------|
| Nieuwe reactie | Een reactie wordt geplaatst. Standaard worden alleen goedgekeurde, niet‑spam reacties geactiveerd. |
| Bijgewerkte reactie | Een reactie wordt bewerkt, goedgekeurd, beoordeeld, vastgezet, vergrendeld, of op een andere manier gewijzigd. |
| Verwijderde reactie | Een reactie wordt verwijderd. |

Elke trigger retourneert de volledige reactie: id, pagin URL en URL‑ID, naam en e‑mail van de reageerder, de reactietekst als markdown en als HTML, stemtellingen, goedkeurings‑ en spam‑vlaggen, de locale, het domein en eventuele vermeldingen. De velden komen overeen met de webhook‑payload die gedocumenteerd is onder Webhooks, Data Structures.

## Opties

**Domein.** Elke trigger heeft een optioneel domeinfilter, waarin de domeinen staan die op uw account zijn geconfigureerd. Laat het leeg om gebeurtenissen van elk domein te ontvangen.

**Include Unapproved and Spam Comments.** Alleen op de Nieuwe reactie‑trigger. Reacties die in de wacht staan voor moderatie of als spam gemarkeerd zijn, worden standaard overgeslagen. Wanneer zo’n reactie later wordt goedgekeurd, wordt de Bijgewerkte reactie‑trigger geactiveerd voor die reactie, zodat een Zap die op elke zichtbaar wordende reactie moet reageren, de Bijgewerkte reactie gebruikt met een filter op het goedgekeurd‑veld.

## Hoe levering werkt

Het inschakelen van een Zap maakt een webhook‑abonnement aan op uw account, zichtbaar op de Webhooks‑pagina met de bron **API**. Het uitschakelen van de Zap verwijdert dit. De eigen limieten van Zapier gelden voor het aantal evenementen dat per minuut wordt geaccepteerd; FastComments probeert een levering die mislukt opnieuw, met een toenemende vertraging, en schakelt een abonnement uit dat zes dagen lang blijft falen. Een uitgeschakeld abonnement kan opnieuw worden ingeschakeld vanaf de Webhooks‑pagina, of schakel de Zap eenvoudig uit en weer in om een nieuw abonnement te maken.

Een account kan maximaal 50 API‑abonnementen bevatten. Elke Zap die een FastComments‑trigger gebruikt, gebruikt er één.

---