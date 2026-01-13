### Understøttelse af mørk tilstand

### Dynamisk mørk tilstand

Hvis dit sites mørke tilstand styres ved at tilføje en `.dark`-klasse til body-elementet, vil Collab Chat UI automatisk respektere dette uden at kræve reinitialisering. Widget'ens stile er designet til at reagere på tilstedeværelsen af denne klasse.

[inline-code-attrs-start title = 'Eksempel på CSS til mørk tilstand'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Din CSS til mørk tilstand */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Brugerdefineret styling med CSS

Du kan tilpasse udseendet af markeringer, chatvinduer og andre elementer ved hjælp af CSS. Widget'en tilføjer specifikke klasser, som du kan målrette i dit stylesheet.

Tekstmarkeringer bruger FastComments' comment bubble-styling system, så eventuelle tilpasninger, du har anvendt på den standard kommenteringswidget, også vil påvirke Collab Chat.

### Tilpasning af topbjælken

Topbjælken viser antallet af brugere online og antallet af diskussioner. Du kan tilpasse dens position ved at angive et brugerdefineret element som `topBarTarget`:

[inline-code-attrs-start title = 'Brugerdefineret topbjælkeplacering'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Eller deaktiver den helt ved at sætte den til `null`:

[inline-code-attrs-start title = 'Deaktiver topbjælken'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Mobiladfærd

På skærme under 768px bredde skifter Collab Chat automatisk til et mobiloptimeret layout. Chatvinduer vises fuldskærm i stedet for at flyde ved siden af teksten, og forsinkelsen ved markering fjernes for mere umiddelbar interaktion.

Denne opførsel er indbygget og kræver ingen konfiguration. Widget'en registrerer skærmstørrelsen automatisk og tilpasser sig derefter.

### Udseende af chatvinduet

Chatvinduer er 410px brede på desktop med en 16px pil, der peger mod den fremhævede tekst. Vinduerne positionerer sig automatisk baseret på tilgængelig viewport-plads og bruger positioneringsklasser som `to-right`, `to-left`, `to-top` og `to-bottom`.

Du kan tilføje brugerdefineret CSS for at justere farver, skrifttyper, afstande eller andre visuelle egenskaber af disse vinduer. Chatvinduerne bruger samme komponentstruktur som den standard FastComments-widget, så de arver alle globale tilpasninger, du har anvendt.

### Lokalisering

Collab Chat understøtter alle de samme lokaliseringsmuligheder som den standard FastComments-widget. Indstil `locale`-optionen for at vise UI-tekst på forskellige sprog:

[inline-code-attrs-start title = 'Indstil lokalitet'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spansk
});
[inline-code-end]

FastComments understøtter dusinvis af sprog. Locale-indstillingen påvirker al UI-tekst, inklusive prompts, knapper og pladsholdertekst.

### Arvede tilpasningsmuligheder

Da Collab Chat udvider den standard kommenteringswidget, arver den alle tilpasningsmuligheder fra basiswidget'en. Dette inkluderer brugerdefinerede CSS-klasser, brugerdefinerede oversættelser, avatar-tilpasning, datoformattering og meget mere.

Se hoveddokumentationen for FastComments-tilpasning for den komplette liste over tilpasningsmuligheder, der er tilgængelige.

### Arbejde med brugerdefinerede skrifttyper

Hvis dit site bruger brugerdefinerede skrifttyper, vil Collab Chat UI arve disse skrifttyper fra din sides CSS. Du kan være nødt til at oprette en widget-tilpasningsregel og `@import` eventuelle skrifttyper i den brugerdefinerede CSS i den regel, hvis du ønsker, at det flydende chatvindue skal bruge de samme skrifttyper.