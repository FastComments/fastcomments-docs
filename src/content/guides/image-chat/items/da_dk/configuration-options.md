### Oversigt

FastComments Image Chat udvider den standard FastComments-kommentar-widget, så den arver alle konfigurationsmuligheder fra base-widget'en samtidig med, at den tilføjer nogle få, som er specifikke for billedannoteringer.

### Påkrævet konfiguration

#### tenantId

Din FastComments Tenant ID er påkrævet. Du kan finde denne i dit [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat-specifikke indstillinger

#### urlId

Som standard genererer Image Chat et unikt id for hver samtale baseret på side-URL'en, billedkilden og X/Y-koordinaterne. Du kan tilsidesætte dette med et brugerdefineret `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Dette er nyttigt, når din URL-struktur kan ændre sig, men du ønsker at bevare de samme samtaler, eller når du ønsker at dele annotationer på tværs af flere sider.

#### chatSquarePercentage

Kontrollerer størrelsen af de klikbare chatmarkører som en procentdel af billedets bredde. Standard er 5%, hvilket betyder, at hver markør er 5% af billedets bredde.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Større, mere synlige markører
});
```

Mindre værdier skaber mindre påtrængende markører, som fungerer bedre til detaljerede billeder. Større værdier gør markørerne lettere at se og klikke på for travle billeder eller for brugere på mobile enheder.

#### hasDarkBackground

Aktivér mørk tilstand-styling, når din side har en mørk baggrund.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

En callback-funktion, der udløses, når antallet af kommentarer ændres. Dette er nyttigt til at opdatere UI-elementer som badges eller sidenavne.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Arvede konfigurationsmuligheder

Da Image Chat udvider den standard kommentar-widget, kan du bruge enhver konfigurationsmulighed fra base FastComments-widget'en. Her er nogle almindeligt brugte muligheder:

#### locale

Indstil sproget for widget-UI'en. FastComments understøtter adskillige sprog.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spansk
});
```

#### readonly

Gør alle samtaler skrivebeskyttede. Brugere kan se eksisterende markører og diskussioner, men kan ikke oprette nye eller svare.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integrer med dit autentificeringssystem ved hjælp af Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO-konfiguration
    }
});
```

Se SSO-dokumentationen for fulde detaljer om autentificeringsmuligheder.

#### maxReplyDepth

Kontroller, hvor mange niveauer dybt svar kan gå. Som standard sætter Image Chat dette til 0, hvilket betyder, at alle kommentarer er flade (ingen indlejrede svar). Du kan ændre dette, hvis du ønsker trådede samtaler.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Tillad 3 niveauer af indlejring
});
```

### Intern konfiguration

Disse indstillinger sættes automatisk af Image Chat og bør ikke overskrives:

`productId` bliver automatisk sat til `2` for Image Chat. `floating-chat`-udvidelsen indlæses automatisk for at levere chatvinduets funktionalitet. Widget'en registrerer automatisk mobile enheder (skærme under 768px i bredden) og tilpasser UI'en tilsvarende med chatvinduer i fuldskærm.

### Fleksibilitet for mål-elementet

Første parameter til `FastCommentsImageChat` kan være enten et `<img>`-element direkte eller et container-element med et billede indeni:

```javascript
// Direkte billedelement
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container med billede indeni
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widget'en finder billedet automatisk, hvis du sender et container-element.

### Komplet eksempel

Her er et eksempel, der viser flere konfigurationsmuligheder samlet:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Din SSO-konfiguration
    },
    maxReplyDepth: 1
});
```

For en komplet liste over alle tilgængelige konfigurationsmuligheder arvet fra base-widget'en, se hoveddokumentationen for FastComments-konfiguration.