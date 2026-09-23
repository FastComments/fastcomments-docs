[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Som standard sporer FastComments ikke, hvem der har set hver kommentar, eller giver nogen statistik omkring dette.

Vi kan dog aktivere denne funktion, hvorefter systemet begynder at spore, når hver bruger ruller til en kommentar.

Når dette sker, vil en tæller ved siden af et øjeikon, der vises på hver kommentar, blive forøget. Tælleren opdateres live og forkortes i henhold til brugerens locale.

Vi kan aktivere dette ved at sætte **enableViewCounts**-flaget til true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Aktivering af visningstællere for kommentarer'; code-example-end]

Dette kan tilpasses uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Widget customization page with the view counts checkbox checked so each comment shows an eye icon and count'; title='Enabling Comment View Counts' app-screenshot-end]

Vi sporer bruger-id'et* der har set kommentaren i en uge, så hvis du ser kommentaren igen inden for den uge, øges tælleren ikke. Hvis du ser kommentaren igen efter ugen er udløbet, vil tælleren blive forøget igen.

- *Bemærk: eller den anonyme sessions-id, eller brugerens IP som en hashværdi.