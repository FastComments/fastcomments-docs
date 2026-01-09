Een `HashTag` object vertegenwoordigt een tag die door een gebruiker kan worden achtergelaten. HashTags kunnen worden gebruikt om naar een extern stuk inhoud te linken of om gerelateerde opmerkingen met elkaar te verbinden.

De structuur van het `HashTag` object is als volgt:

[inline-code-attrs-start title = 'HashTag-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Moet beginnen met de "#" of het gewenste teken. **/
    tag: string
    /** Een optionele URL waar de hashtag naartoe kan verwijzen. In plaats van opmerkingen te filteren op hashtag, zal de UI hiernaartoe doorverwijzen bij een klik. **/
    url?: string
    /** ALLEEN-LEZEN **/
    createdAt: string
}
[inline-code-end]

Opmerkingen:

- In sommige API endpoints zie je dat de hashtag in de URL wordt gebruikt. Vergeet niet waarden te URI-encoderen. Bijvoorbeeld, `#` moet in plaats daarvan worden weergegeven als `%23`.
- Sommige van deze velden zijn gemarkeerd als `READONLY` - deze worden door de API geretourneerd maar kunnen niet worden ingesteld.