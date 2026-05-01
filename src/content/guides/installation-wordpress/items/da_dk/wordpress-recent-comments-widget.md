Widget'en "Seneste kommentarer" viser de seneste kommentarer, der er indsendt på hele dit websted. Den er nyttig i sidepaneler, sidefødder eller hvor som helst, hvor du vil fremhæve ny aktivitet for at opmuntre til videre læsning.

## Indstillinger

- **Titel** (valgfri): Overskriften vist over listen. Standard er "Seneste kommentarer".
- **Antal** (valgfri): Hvor mange kommentarer der skal vises. Interval 1 til 50. Standard er 5.

## Sådan tilføjer du den

### I et indlæg eller en side

I blokeditoren, tilføj en **Kortkode**-blok og indsæt:

[inline-code-attrs-start title = 'Kortkode: Seneste kommentarer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Attributten `count` accepterer enhver værdi mellem 1 og 50.

### I en sidepanel eller sidefod (klassiske temaer)

Gå til **Udseende > Widgets** i din WordPress-administration. Fra blokindsætteren, søg efter "FastComments" og vælg **FastComments: Seneste kommentarer**. Træk den ind i en sidepanel, header eller sidefod, og konfigurer derefter titel og antal fra widget-panelet.

### I et bloktema (Fuld redigering af websted)

Åbn Sideeditoren under **Udseende > Editor**. Naviger til den skabelondel, hvor widget'en skal vises, indsæt en **Legacy Widget**-blok, og vælg **FastComments: Seneste kommentarer** fra dropdown-menuen.

## Fejlfinding

Widget'en gengives kun efter FastComments-opsætningen er fuldført og et tenant ID er gemt. Hvis widget-området er tomt, fuldfør opsætningen under **FastComments** i WordPress-administrationen og genindlæs siden.