Hvis du har installeret FastComments fra Shopify App Store, bliver dit shop-domæne automatisk føjet til tenantens autoriserede domæner, og du bør ikke få en domænefejl. Denne side gælder, hvis du fulgte den manuelle installationsvej, eller hvis din butiksside bliver serveret på et brugerdefineret domæne, der ikke var registreret hos Shopify på det tidspunkt, hvor appen blev installeret.

Du kan få en autorisationsfejl, der ser sådan ud:

<div class="screenshot white-bg">
    <div class="title">Domænekonfiguration mangler</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-webflow-step-5.png" alt="Domænekonfiguration mangler" />
</div>

Det betyder, at FastComments ikke genkender det domæne, som widgetten indlæses på, som autoriseret for din tenant.

For at rette det skal du tilføje domænet til din FastComments-konto: [Konfigurer domæner](https://fastcomments.com/auth/my-account/configure-domains).