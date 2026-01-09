FastComments is een gelokaliseerd platform. Al onze widgets, e-mails en meldingen zijn gelokaliseerd.

Gelokaliseerd betekent dat we een andere taal, en opmaak, tonen, gebaseerd op de locatie van de gebruiker
en de voorkeurstaal. We bepalen dit op basis van de informatie die de browser van de gebruiker ons verstrekt.

We kunnen de tekst in de e-mail aanpassen door naar het `Translations` tabblad te gaan, een `Locale`
te selecteren en de tekst te bewerken. Tekst die is gewijzigd ten opzichte van de standaard wordt in de UI omlijnd. Je mag
tussen locales wisselen en aan het einde opslaan, zonder wijzigingen te verliezen.

Gelokaliseerde tekst wordt benaderd via het `TEXT` object, bijvoorbeeld: `<%= TEXT.INTRO %>`.

### SSO-opmerking

Voor SSO-integraties, als `locale` niet is opgegeven, wordt deze elke keer bijgewerkt wanneer de gebruiker
het commentaar-widget opent met een andere locale. Dit betekent dat hun taalvoorkeur
automatisch wordt bijgewerkt, en toekomstige e-mails in die locale worden verzonden.

Dit kan ook handmatig worden ingesteld door `locale` op te geven in de SSO-payload.