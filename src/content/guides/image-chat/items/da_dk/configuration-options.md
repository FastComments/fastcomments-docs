### Oversigt

FastComments Image Chat udvider standard FastComments-kommentarwidgetten, så den arver alle konfigurationsmulighederne fra basiswidgetten og samtidig tilføjer et par specifikke for billedannotationer.

### Påkrævet konfiguration

#### tenantId

Dit FastComments Tenant ID er påkrævet. Du kan finde det i dit [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat-specifikke indstillinger

#### urlId

Som standard genererer Image Chat en unik identifikator for hver samtale baseret på sidens URL, billedkilden og X/Y-koordinaterne. Du kan tilsidesætte dette med en brugerdefineret `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Dette er nyttigt, når din URL-struktur muligvis ændres, men du vil beholde de samme samtaler, eller når du vil dele annotationer på tværs af flere sider.

#### chatSquarePercentage

Styrer størrelsen af de klikbare chat-markører som en procentdel af billedets bredde. Standardværdien er 5 %, hvilket betyder, at hver markør er 5 % af billedets bredde.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Større, mere synlige markører
});
```

Mindre værdier skaber mindre påtrængende markører, der fungerer bedre til detaljerede billeder. Større værdier gør markørerne lettere at se og klikke på travle billeder eller for brugere på mobile enheder.

#### hasDarkBackground

Aktivér mørk tilstand, når din side har en mørk baggrund.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

En callback-funktion, der udløses, hver gang antallet af kommentarer ændres. Dette er nyttigt til at opdatere UI-elementer som badges eller sidetitler.

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

Da Image Chat udvider standardkommentarwidgetten, kan du bruge enhver konfigurationsmulighed fra basis-FastComments-widgetten. Her er nogle almindeligt anvendte indstillinger:

#### locale

Indstil sproget for widgettens UI. FastComments understøtter snesevis af sprog.

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

#### sso og simpleSSO

Integrér med dit autentificeringssystem ved hjælp af Single Sign-On.

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

Styr, hvor mange niveauer dybe svar kan gå. Som standard sætter Image Chat denne til 0, hvilket betyder, at alle kommentarer er flade (ingen indlejrede svar). Du kan ændre dette, hvis du ønsker trådede samtaler.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Tillad 3 niveauer af indlejring
});
```

### Intern konfiguration

Disse indstillinger sættes automatisk af Image Chat og bør ikke tilsidesættes:

`productId` sættes automatisk til `2` for Image Chat. Udvidelsen `floating-chat` indlæses automatisk for at levere chatvinduesfunktionaliteten. Widgetten registrerer automatisk mobile enheder (skærme under 768px i bredden) og tilpasser UI'en i overensstemmelse hermed med chatvinduer i fuldskærm.

### Fleksibilitet for målelementet

Den første parameter til `FastCommentsImageChat` kan enten være et `<img>`-element direkte eller et containerelement med et billede indeni:

```javascript
// Direkte billedelement
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container med billede indeni
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widgetten finder billedet automatisk, hvis du sender et containerelement.

### Komplet eksempel

Her er et eksempel, der viser flere konfigurationsmuligheder sammen:

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

For en komplet liste over alle tilgængelige konfigurationsmuligheder, der er arvet fra basiswidgetten, se hoveddokumentationen for FastComments-konfiguration.