De widget Recente Reacties toont de meest recent geplaatste reacties op je hele site. Het is handig in zijbalken, voetteksten of overal waar je recente activiteit wilt tonen om verder lezen aan te moedigen.

## Opties

- **Titel** (optioneel): De kop die boven de lijst wordt weergegeven. Standaard "Recente Reacties".
- **Aantal** (optioneel): Hoeveel reacties er worden weergegeven. Bereik 1 tot 50. Standaard 5.

## Hoe toe te voegen

### In een bericht of pagina

Voeg in de block editor een **Shortcode**-blok toe en plak:

[inline-code-attrs-start title = 'Shortcode Recente Reacties'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Het `count` attribuut accepteert een waarde tussen 1 en 50.

### In een zijbalk of voettekst (klassieke thema's)

Ga naar **Weergave > Widgets** in je WordPress-beheer. Zoek in de block-invoeger naar "FastComments" en kies **FastComments: Recente Reacties**. Sleep het naar een zijbalk-, header- of voettekstgebied en stel vervolgens de titel en het aantal in via het widgetpaneel.

### In een blockthema (Full Site Editing)

Open de **Site-editor** onder **Weergave > Editor**. Navigeer naar het template-gedeelte waar de widget moet verschijnen, voeg een **Legacy Widget**-blok in en selecteer **FastComments: Recente Reacties** uit het dropdownmenu.

## Problemen oplossen

De widget wordt alleen weergegeven nadat de FastComments-setup is voltooid en een tenant-ID is opgeslagen. Als het widgetgebied leeg is, voltooi dan de setup onder **FastComments** in het WordPress-beheer en laad de pagina opnieuw.