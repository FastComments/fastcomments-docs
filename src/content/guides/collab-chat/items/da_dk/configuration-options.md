### Oversigt

FastComments Collab Chat udvider den standard FastComments-kommenteringswidget, så den arver alle konfigurationsmuligheder fra basis-widgeten, samtidig med at den tilføjer nogle få specifikke muligheder til tekstannoteringer.

### Påkrævet konfiguration

#### tenantId

Dit FastComments Tenant ID er påkrævet. Du kan finde dette i dit [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Specifikke muligheder for Collab Chat

#### urlId

Som standard genererer Collab Chat en unik identifikator for hver samtale baseret på side-URL'en, DOM-stien til elementet og det valgte tekstområde. Du kan tilsidesætte dette med en brugerdefineret `urlId`.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Dette er nyttigt, når din URL-struktur kan ændre sig, men du ønsker at bevare de samme samtaler, eller når du ønsker at dele annoteringer på tværs af flere sider.

#### topBarTarget

Styrer visningen af topbaren, som viser antal brugere og antal diskussioner. Sæt til `null` for helt at deaktivere topbaren, eller angiv et DOM-element for at rendere den i en specifik placering.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Deaktiver topbaren
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Vis topbaren i en brugerdefineret placering
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Aktivér mørktilstand-styling, når din side har en mørk baggrund. Denne detektion er automatisk, men det kan være ønskeligt at tilsidesætte den.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

En callback-funktion, der kaldes, hver gang kommentartællingen ændres. Dette er nyttigt til at opdatere UI-elementer som badges eller sidetitler.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Arvede konfigurationsmuligheder

Da Collab Chat udvider den standard kommenteringswidget, kan du bruge enhver konfigurationsmulighed fra basis FastComments-widgeten. Her er nogle ofte anvendte muligheder:

#### locale

Indstil sproget for widgetens brugerflade. FastComments understøtter dusinvis af sprog.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spansk
});
[inline-code-end]

#### readonly

Gør alle samtaler skrivebeskyttede. Brugere kan se eksisterende annoteringer, men kan ikke oprette nye eller svare.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integrer med dit autentificeringssystem ved hjælp af Single Sign-On.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO-konfiguration
    }
});
[inline-code-end]

Se SSO-dokumentationen for fulde detaljer om godkendelsesmuligheder.

#### maxReplyDepth

Styr, hvor mange niveauer svar kan gå i dybden. Som standard sætter Collab Chat dette til 0, hvilket betyder, at alle kommentarer er flade (ingen indlejrede svar). Du kan ændre dette, hvis du ønsker trådede samtaler.

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Tillad 3 niveauer af indlejring
});
[inline-code-end]

### Intern konfiguration

Disse muligheder sættes automatisk af Collab Chat og bør ikke tilsidesættes:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Komplet eksempel

Her er et eksempel, der viser flere konfigurationsmuligheder samlet:

[inline-code-attrs-start title = "Eksempel på konfiguration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Din SSO-konfiguration
    },
    maxReplyDepth: 1
});
[inline-code-end]

For en komplet liste over alle tilgængelige konfigurationsmuligheder arvet fra basis-widgeten, se hoveddokumentationen for FastComments-konfiguration.