[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Som standard udføres formateringsfunktionerne i FastComments ved at tilføje synlige anker‑tags som `<b></b>` omkring din tekst. Når du klikker på værktøjslinjen
eller bruger genveje, sker dette automatisk. Nogle fællesskaber kan dog ønske at bruge formatering uden anker‑tags. Dette kaldes at aktivere
WYSIWYG‑editoren (what you see is what you get). Denne editor ser nøjagtig ud som standardeditoren, men indlæser ekstra kode, så brugere kan gøre deres tekst fed, understreget osv. uden synlige anker‑tags.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Aktivering af WYSIWYG-redigering'; code-example-end]

Dette kan også gøres uden kode. På widget‑tilpasningssiden, se indstillingen "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Widget-tilpasningsside med Enable Advanced Formatting afkrydsningsfeltet markeret for at aktivere WYSIWYG-editoren'; title='Aktiver WYSIWYG' app-screenshot-end]