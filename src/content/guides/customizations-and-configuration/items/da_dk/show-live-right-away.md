[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Som standard er live‑kommentering aktiveret. Det betyder, at hvis kommentarer tilføjes, slettes, redigeres eller fastgøres, skal ændringerne vises for alle brugere, der ser kommentarfæden på samme tid.

Dog vil de nye kommentarer som standard vises under en dynamisk vist knap med tekst, der ligner "Vis 2 nye kommentarer".

Hvis de nye kommentarer er svar direkte på siden, vil knappen vises øverst i kommentarfæden. Hvis de er svar på en bestemt kommentar, vil knappen vises under den kommentar.

Dette er for at forhindre, at sidens størrelse konstant ændrer sig for brugeren, hvilket potentielt kan forårsage frustration, når man forsøger at gribe fat i rullebjælken.

For nogle anvendelsestilfælde, som livebudgivning eller online‑begivenheder, er dette ikke den ønskede adfærd – du vil måske have kommentarfunktionen til at ligne en "chat"-boks, hvor nye kommentarer "vises med det samme".

Derfor er navnet på flaget, der aktiverer denne funktion: **showLiveRightAway**.

Vi kan aktivere den som følger:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Vis live kommentarer med det samme'; code-example-end]

Dette kan tilpasses uden kode på widget‑tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Indstillingen for at skjule live kommentarer er slået til, så nye kommentarer vises øjeblikkeligt i stedet for bag en knap'; title='Vis live kommentarer med det samme' app-screenshot-end]