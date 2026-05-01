---
De widget Recent Discussions toont de pagina's op uw site met de meest recente reactie-activiteit. Het is handig om threads te benadrukken waar nog reacties aan worden toegevoegd, zodat bezoekers terug kunnen springen in actieve gesprekken in plaats van op rustige pagina's te belanden.

## Opties

- **Title** (optioneel): De kop die boven de lijst wordt weergegeven. Standaard "Recente Discussies".
- **Count** (optioneel): Hoeveel discussies er worden weergegeven. Bereik 1 tot 50. Standaard 20.

## Hoe het toe te voegen

### Binnen een bericht of pagina

In de blokeditor, voeg een **Shortcode**-blok toe en plak:

[inline-code-attrs-start title = 'Shortcode Recente Discussies'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

De `count`-attribuut accepteert elke waarde tussen 1 en 50.

### In een zijbalk of voettekst (klassieke thema's)

Ga naar **Weergave > Widgets** in uw WordPress-beheer. Zoek in de blokinvoeger naar "FastComments" en kies **FastComments: Recente Discussies**. Sleep het naar een zijbalk, header of voettekstgebied en configureer vervolgens de titel en het aantal in het widgetpaneel.

### In een blockthema (Volledige sitebewerking)

Open de **Site-editor** onder **Weergave > Editor**. Navigeer naar het sjabloondeel waar de widget moet verschijnen, voeg een **Legacy Widget**-blok in en selecteer **FastComments: Recente Discussies** uit de keuzelijst.

## Problemen oplossen

De widget wordt pas weergegeven nadat de FastComments-configuratie is voltooid en een tenant-ID is opgeslagen. Als het widgetgebied leeg is, voltooi dan de installatie onder **FastComments** in het WordPress-beheer en herlaad de pagina.

Als de volgorde van discussies verouderd lijkt, controleer dan of de onderliggende pagina's klaar zijn met synchroniseren in het FastComments-dashboard. De widget leest livegegevens, dus recent geïmporteerde reacties kunnen even duren voordat ze zichtbaar worden.