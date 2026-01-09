Een `NotificationCount` object vertegenwoordigt het aantal ongelezen meldingen en metadata voor een gebruiker.

Als er geen ongelezen meldingen zijn, bestaat er geen `NotificationCount` voor de gebruiker.

`NotificationCount`-objecten worden automatisch aangemaakt en kunnen niet via de API worden gemaakt. Ze verlopen ook na één jaar.

U kunt het aantal ongelezen meldingen van een gebruiker wissen door hun `NotificationCount` te verwijderen.

De structuur van het `NotificationCount`-object is als volgt:

[inline-code-attrs-start title = 'NotificationCount-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // gebruikers-id
    count: number
    createdAt: string // datumstring
    expireAt: string // datumstring
}
[inline-code-end]

---