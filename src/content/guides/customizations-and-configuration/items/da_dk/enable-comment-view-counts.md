[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Som standard sporer FastComments ikke, hvem der har set hver kommentar, eller giver nogen statistik omkring dette.

Vi kan dog aktivere denne funktion, og så vil systemet begynde at spore, når hver bruger scroller hen til en kommentar.

Når dette sker, bliver et tal ved siden af et øjeikon vist på hver kommentar øget. Tallet opdateres live og forkortes i henhold til brugerens lokalitet.

Vi kan aktivere dette ved at sætte flaget **enableViewCounts** til true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Dette kan tilpasses uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Vi sporer bruger-id'et* for den, der så kommentaren, så hvis du ser kommentaren igen, bliver det ikke talt op. Hvis du ser kommentaren igen
efter to år, vil tællingen øges.

- *Bemærk: eller det anonyme session-id, eller brugerens IP som en hash-værdi.