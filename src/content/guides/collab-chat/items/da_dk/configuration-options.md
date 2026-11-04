### Oversigt

FastComments Collab Chat udvider standard FastComments-kommentarwidgeten, så den arver alle konfigurationsmulighederne fra basiswidgeten og samtidig tilføjer et par specifikke for tekstannotationer.

### Påkrævet konfiguration

#### tenantId

Dit FastComments Tenant ID er påkrævet. Du kan finde det i dit [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat-specifikke indstillinger

#### urlId

Som standard genererer Collab Chat et unikt id for hver samtale baseret på sidens URL, DOM-stien til elementet og det valgte tekstinterval. Du kan overskrive dette med en brugerdefineret `urlId`.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Dette er nyttigt, når din URL-struktur kan ændre sig, men du vil beholde de samme samtaler, eller når du vil dele annotationer på tværs af flere sider.

#### topBarTarget

Styrer visningen af topbjælken, som viser antal brugere og antal diskussioner. Sæt den til `null` for at deaktivere topbjælken helt, eller angiv et DOM-element for at gengive den på en bestemt placering.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Aktivér mørk tilstand, når din side har en mørk baggrund. Denne detektering er automatisk, men det kan være ønskeligt at overskrive den.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

En callback-funktion, der udløses, hver gang antallet af kommentarer ændres. Dette er nyttigt til at opdatere UI-elementer som badges eller sidetitler.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Da Collab Chat udvider standardkommentarwidgeten, kan du bruge enhver konfigurationsmulighed fra basis-FastComments-widgeten. Her er nogle almindeligt anvendte indstillinger:

#### locale

Indstil sproget for widgetens UI. FastComments understøtter dusinvis af sprog.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Gør alle samtaler skrivebeskyttede. Brugere kan se eksisterende annotationer, men kan ikke oprette nye eller svare.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integrér med dit autentificeringssystem ved hjælp af Single Sign-On.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Se SSO-dokumentationen for fulde detaljer om autentificeringsmuligheder.

#### maxReplyDepth

Styr, hvor mange niveauer dybe svar kan gå. Som standard sætter Collab Chat denne til 0, hvilket betyder, at alle kommentarer er flade (ingen indlejrede svar). Du kan ændre dette, hvis du vil have trådede samtaler.

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Intern konfiguration

Disse indstillinger sættes automatisk af Collab Chat og bør ikke overskrives:

`productId` sættes automatisk til `3` for Collab Chat. Udvidelsen `floating-chat` indlæses automatisk for at levere chatvinduets funktionalitet. Widgeten detekterer automatisk mobile enheder (skærme under 768px brede) og tilpasser UI'en i overensstemmelse hermed.

### Komplet eksempel

Her er et eksempel, der viser flere konfigurationsmuligheder sammen:

[inline-code-attrs-start title = "Konfigurationseksempel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

For en komplet liste over alle tilgængelige konfigurationsmuligheder, der er arvet fra basiswidgeten, se hoveddokumentationen for FastComments-konfiguration.