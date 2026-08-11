[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Som standard sporer FastComments ikke, hvem der har set hver kommentar, eller giver nogen statistik omkring dette.

Vi kan dog aktivere denne funktion, hvorefter systemet begynder at spore, når hver bruger ruller til en kommentar.

Når dette sker, vil en tæller ved siden af et øje‑ikon, der vises på hver kommentar, blive øget. Tælleren opdateres live og forkortes i henhold til brugerens locale.

Vi kan aktivere dette ved at sætte **enableViewCounts** flaget til true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Aktivering af kommentarvisningstællere'; code-example-end]

Dette kan tilpasses uden kode på widget‑tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Widget-tilpasningsside med afkrydsningsfeltet for visningstællere markeret, så hver kommentar viser et øje-ikon og en tæller'; title='Aktivering af kommentarvisningstællere' app-screenshot-end]

Vi sporer bruger‑id* som har set kommentaren, så hvis du ser kommentaren igen, øges den ikke. Hvis du ser kommentaren igen efter to år, vil tælleren øges mere.

- *Bemærk: eller den anonyme sessions‑id, eller brugerens IP som en hash‑værdi.