[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Som standard udføres formateringsfunktionerne i FastComments ved at tilføje synlige anker-tags som `<b></b>` omkring din tekst. At klikke på værktøjslinjen eller bruge genvejstaster gør dette for dig. Dog kan nogle fællesskaber ønske at vælge at bruge formatering uden anker-tags. Dette kaldes at aktivere WYSIWYG (hvad du ser er hvad du får)-editoren. Denne editor ser præcis ud som standardeditoren, bortset fra at den indlæser noget ekstra kode, som tillader brugere at gøre tekst fed, understrege osv. uden synlige anker-tags.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden skal du se indstillingen "Aktivér avanceret formatering".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---